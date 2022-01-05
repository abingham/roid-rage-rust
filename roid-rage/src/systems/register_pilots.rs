use crate::components::Pilot;
use roid_rage_grpc::roid_rage::pilot_registrar_server::{PilotRegistrar, PilotRegistrarServer};
use roid_rage_grpc::roid_rage::{RegistrationRequest, RegistrationResponse};
use specs::{Entities, System, WriteStorage};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use tokio::runtime::Runtime;
use tonic::{transport::Server, Code, Request, Response, Status};

// What needs to happen here?
// 1. A task/thread for listening for pilot registrations

// TODO: Consider renaming to PilotRegistration or something like that.
pub struct RegisterPilotsSystem {
    rt: Runtime,
    rx: Receiver<String>,
}

impl RegisterPilotsSystem {
    pub fn new(url: &str) -> Result<RegisterPilotsSystem, std::io::Error> {
        // TODO: Should the runtime be a resource in the world?
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let (tx, rx) = channel();

        let system = RegisterPilotsSystem { rt: rt, rx: rx };

        // Launch the registrar listener.
        println!("new pilot reg system");
        system.rt.spawn(listen(String::from(url), tx));

        Ok(system)
    }
}

impl<'s> System<'s> for RegisterPilotsSystem {
    type SystemData = (
        WriteStorage<'s, Pilot>,
        Entities<'s>,
    );

    fn run(&mut self, (mut pilots, entities): Self::SystemData) {
        // TODO: What if the URL is already registered? Ignore it.
        loop {
            match self.rx.try_recv() {
                Err(_) => break,
                Ok(pilot_url) => {
                    let new_entity = entities.create();
                    match pilots.insert(new_entity, Pilot::new(&pilot_url)) {
                        Err(_) => println!("oops! Trouble creating pilot"),
                        Ok(_) => println!("new pilot"),
                    }
                }
            }
        }
    }
}

// Listen for registrations on a URL, publishing them to a channel.
async fn listen(url: String, tx: Sender<String>) -> Result<(), tonic::transport::Error> {
    let registrar = Registrar { tx: Mutex::new(tx) };
    let svc = PilotRegistrarServer::new(registrar);
    // TODO: This parse().unwrap() call is bad. Callers should pass in a SocketAddr.
    println!("Listening for pilot registration on {:?}", url);
    let result = Server::builder()
        .add_service(svc)
        .serve(url.parse().unwrap())
        .await?;
    println!("Registration listener closing");
    Ok(result)
}

/// Stores new registrations from pilots.
/// This is the implementation of the grpc PilotRegistrar.
struct Registrar {
    tx: Mutex<Sender<String>>,
}

#[tonic::async_trait]
impl PilotRegistrar for Registrar {
    async fn register(
        &self,
        request: Request<RegistrationRequest>,
    ) -> Result<Response<RegistrationResponse>, Status> {
        let registration = request.get_ref();

        {
            let send = self.tx.lock().unwrap();
            send.send(registration.url.clone())
                .or_else(|e| Err(Status::new(Code::Internal, e.to_string())))?;
        }

        Ok(Response::new(RegistrationResponse {}))
    }
}
