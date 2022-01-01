#![feature(proc_macro_hygiene, decl_macro)]

// mod targeting;

extern crate nalgebra;

// use crate::targeting::find_target;
use tonic::{transport::Server, Request, Response, Status};

use roid_rage_grpc::roid_rage::pilot_server::{
    Pilot, PilotServer
};

use roid_rage_grpc::roid_rage::{
  Command, GameState, Rotation
};

#[derive(Default)]
struct SimplePilot {}

#[tonic::async_trait]
impl Pilot for SimplePilot {
    async fn get_command(
        &self,
        _request: Request<GameState>,
    ) -> Result<Response<Command>, Status>
    {
        let command = Command {
            fire: false,
            rotation: Rotation::None as i32,
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

