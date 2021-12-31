#![feature(proc_macro_hygiene, decl_macro)]

mod targeting;

extern crate nalgebra;

use crate::targeting::find_target;
use tonic::{transport::Server, Request, Response, Status};

use roid_rage_grpc::roid_rage::pilot_server::{
    Pilot, PilotServer
};

use roid_rage_grpc::roid_rage::{
  Command, GameState, Rotation
};

// #[post("/", format = "json", data = "<game_state>")]
// fn update(game_state: Json<GameState>) -> Json<Command> {
//     let target = find_target(
//         &game_state.firing_position,
//         game_state.ship.cannon.bullet_speed,
//         &game_state.field,
//         &game_state.roids,
//     );

//     let cmd = match target {
//         Some(_bearing) => Command {
//             fire: true,
//             rotation: Rotation::Clockwise,
//             thrusters: false,
//         },
//         None => Command {
//             fire: false,
//             rotation: Rotation::None,
//             thrusters: false,
//         },
//     };

//     Json(cmd)
// }

#[derive(Default)]
struct SimplePilot {}

#[tonic::async_trait]
impl Pilot for SimplePilot {
    async fn get_command(
        &self,
        request: Request<GameState>,
    ) -> Result<Response<Command>, Status>
    {
        let command = Command {
            fire: false,
            rotation: 0, // TODO: Rotation::None.,
            thrusters: false
        };
        Ok(Response::new(command))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // dotenv().ok();
    // env_logger::init();

    // info!("Starting simple-pilot");

    let addr = "[::1]:50051".parse().unwrap();
    // let addr = std::env::var("GRPC_SERVER_ADDRESS")?.parse()?;

    let pilot = SimplePilot::default();
    let svc = PilotServer::new(pilot);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

