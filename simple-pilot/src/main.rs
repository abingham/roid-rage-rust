// mod targeting;

extern crate nalgebra;

// use crate::targeting::find_target;
use tonic::{Request, Response, Status};

use roid_rage_grpc::roid_rage::pilot_server::Pilot;

use roid_rage_grpc::roid_rage::{Command, GameState, Rotation};

#[derive(Default)]
struct SimplePilot {}

#[tonic::async_trait]
impl Pilot for SimplePilot {
    async fn get_command(&self, _request: Request<GameState>) -> Result<Response<Command>, Status> {
        let command = Command {
            fire: false,
            rotation: Rotation::None as i32,
            thrusters: false,
        };
        Ok(Response::new(command))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pilot = SimplePilot::default();
    pilot_lib::pilot_base::pilot_main(pilot).await
}
