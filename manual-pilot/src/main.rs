use device_query::{DeviceQuery, DeviceState, Keycode};
use roid_rage_grpc::roid_rage as rpc;
use roid_rage_grpc::roid_rage::pilot_server::{Pilot, PilotServer};
use tonic::{transport::Server, Request, Response, Status};

struct PilotState {
    device_state: DeviceState,
}

impl PilotState {
    fn new() -> PilotState {
        PilotState {
            device_state: DeviceState::new(),
        }
    }
}

#[tonic::async_trait]
impl Pilot for PilotState {
    async fn get_command(
        &self,
        _request: Request<rpc::GameState>,
    ) -> Result<Response<rpc::Command>, Status> {
        let mut cmd = rpc::Command::null();

        let keys: Vec<Keycode> = self.device_state.get_keys();

        for key in keys {
            cmd = match key {
                Keycode::Right => rpc::Command {
                    rotation: rpc::Rotation::Clockwise as i32,
                    ..cmd
                },
                Keycode::Left => rpc::Command {
                    rotation: rpc::Rotation::Counterclockwise as i32,
                    ..cmd
                },
                Keycode::Up => rpc::Command {
                    thrusters: true,
                    ..cmd
                },
                Keycode::Space => rpc::Command { fire: true, ..cmd },
                _ => cmd,
            }
        }

        Ok(Response::new(cmd))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: The address of this pilot should be a command line option or something.
    let pilot_address = "[::1]:50052";
    let pilot_url = format!("http://{}", pilot_address);

    let pilot = PilotState::new();
    let svc = PilotServer::new(pilot);

    // Run the pilot service
    let handle = tokio::spawn(
        Server::builder()
            .add_service(svc)
            .serve(pilot_address.parse().unwrap()),
    );

    // TODO: The address of the server should be a command line option or something.
    let server_address = "http://[::1]:50051";

    // First, register with game
    let mut client =
        rpc::pilot_registrar_client::PilotRegistrarClient::connect(server_address).await?;
    let request = rpc::RegistrationRequest {
        url: pilot_url
    };
    client.register(request).await?;

    // dotenv().ok();
    // env_logger::init();

    // info!("Starting simple-pilot");

    handle.await?;

    Ok(())
}
