use roid_rage_grpc::roid_rage as rpc;
use roid_rage_grpc::roid_rage::pilot_server::{Pilot, PilotServer};
use tonic::{transport::Server, Request, Response, Status};
use device_query::{DeviceQuery, DeviceState, Keycode};

struct PilotState {
    device_state: DeviceState
}

impl PilotState {
    fn new() -> PilotState {
        PilotState {
            device_state: DeviceState::new()
        }
    }
}


#[tonic::async_trait]
impl Pilot for PilotState {
    async fn get_command(&self, _request: Request<rpc::GameState>) -> Result<Response<rpc::Command>, Status> {
        let mut cmd = rpc::Command::null();

        let keys: Vec<Keycode> = self.device_state.get_keys();

        for key in keys {
            cmd = match key {
                Keycode::Right => rpc::Command {rotation: rpc::Rotation::Clockwise as i32, ..cmd},
                Keycode::Left => rpc::Command {rotation: rpc::Rotation::Counterclockwise as i32, ..cmd},
                Keycode::Up => rpc::Command { thrusters: true, ..cmd},
                Keycode::Space => rpc::Command {fire: true, ..cmd},
                _ => cmd 
            }
        }

        Ok(Response::new(cmd))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // dotenv().ok();
    // env_logger::init();

    // info!("Starting simple-pilot");

    let addr = "[::1]:50051".parse().unwrap();
    // let addr = std::env::var("GRPC_SERVER_ADDRESS")?.parse()?;

    let pilot = PilotState::new();
    let svc = PilotServer::new(pilot);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}