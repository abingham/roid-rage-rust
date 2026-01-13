use crate::components::{FireTimer, Pilot};
use crate::settings::Settings;
use roid_rage_grpc::roid_rage::pilot_registrar_server::{PilotRegistrar, PilotRegistrarServer};
use roid_rage_grpc::roid_rage::{RegistrationRequest, RegistrationResponse};
use specs::prelude::*;
use specs::{Entities, System, World, WriteStorage};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use tokio::sync::oneshot;
use tonic::{transport::Server, Code, Request, Response, Status};

// What needs to happen here?
// 1. A task/thread for listening for pilot registrations

// TODO: Consider renaming to PilotRegistration or something like that.
pub struct RegisterPilotsSystem {
    rx: Receiver<String>,
    tx: Sender<String>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl RegisterPilotsSystem {
    pub fn new() -> RegisterPilotsSystem {
        let (tx, rx) = channel();

        RegisterPilotsSystem {
            rx: rx,
            tx: tx,
            shutdown_tx: None,
        }
    }
}

impl<'s> System<'s> for RegisterPilotsSystem {
    type SystemData = (WriteStorage<'s, Pilot>, WriteStorage<'s, FireTimer>, Entities<'s>);
    fn setup(&mut self, world: &mut World) {
        let runtime = world.read_resource::<tokio::runtime::Runtime>();
        let settings = world.read_resource::<Settings>();
        let addr = settings.pilot_registration_url;
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);
        runtime.spawn(listen(addr, self.tx.clone(), shutdown_rx));
    }

    fn dispose(self, _world: &mut World) {
        if let Some(shutdown_tx) = self.shutdown_tx {
            let _ = shutdown_tx.send(());
        }
    }

    fn run(&mut self, (mut pilots, mut fire_timers, entities): Self::SystemData) {
        loop {
            match self.rx.try_recv() {
                Err(_) => break,
                Ok(pilot_url) => {
                    if (&pilots).join().any(|pilot| pilot.url == pilot_url) {
                        println!("pilot already registered: {}", pilot_url);
                        continue;
                    }
                    let new_entity = entities.create();
                    match pilots.insert(new_entity, Pilot::new(&pilot_url)) {
                        Err(_) => println!("oops! Trouble creating pilot"),
                        Ok(_) => match fire_timers.insert(new_entity, FireTimer(0.0)) {
                            Err(_) => println!("oops! Trouble creating fire timer"),
                            Ok(_) => println!("new pilot"),
                        },
                    }
                }
            }
        }
    }
}

// Listen for registrations on a URL, publishing them to a channel.
async fn listen(
    addr: SocketAddr,
    tx: Sender<String>,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), tonic::transport::Error> {
    let registrar = Registrar { tx: Mutex::new(tx) };
    let svc = PilotRegistrarServer::new(registrar);
    println!("Listening for pilot registration on {:?}", addr);
    let shutdown = async {
        let _ = shutdown_rx.await;
    };
    let result = Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, shutdown)
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
