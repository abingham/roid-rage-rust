use crate::components::Pilot;
use crate::settings::Settings;
use roid_rage_grpc::roid_rage::pilot_registrar_server::{PilotRegistrar, PilotRegistrarServer};
use roid_rage_grpc::roid_rage::{RegistrationRequest, RegistrationResponse};
use specs::prelude::*;
use specs::{Entities, System, World, WriteStorage};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use tonic::{transport::Server, Code, Request, Response, Status};

// What needs to happen here?
// 1. A task/thread for listening for pilot registrations

// TODO: Consider renaming to PilotRegistration or something like that.
pub struct RegisterPilotsSystem {
    rx: Receiver<String>,
    tx: Sender<String>,
}

impl RegisterPilotsSystem {
    pub fn new() -> RegisterPilotsSystem {
        let (tx, rx) = channel();

        RegisterPilotsSystem { rx: rx, tx: tx }
    }
}

impl<'s> System<'s> for RegisterPilotsSystem {
    type SystemData = (WriteStorage<'s, Pilot>, Entities<'s>);
    fn setup(&mut self, world: &mut World) {
        let runtime = world.read_resource::<tokio::runtime::Runtime>();
        let settings = world.read_resource::<Settings>();
        runtime.spawn(listen(
            String::from(&settings.pilot_registration_url),
            self.tx.clone(),
        ));
    }

    fn dispose(self, _world: &mut World) {
        // TODO: Kill the listener?
    }

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
