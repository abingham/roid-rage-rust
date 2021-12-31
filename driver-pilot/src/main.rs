#![feature(proc_macro_hygiene, decl_macro)]

extern crate nalgebra;

use float_cmp::ApproxEqRatio;
use rand::prelude::*;
use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::Mutex;
use sted::Bearing;
use sted::Velocity;
use tonic::{transport::Server, Request, Response, Status};

use roid_rage_grpc::roid_rage::pilot_server::{Pilot, PilotServer};

use roid_rage_grpc::roid_rage::{Command, GameState, Rotation};

enum Activity {
    Accelerate(usize),
    Stop,
}

struct PilotState {
    activity: Arc<Mutex<Activity>>,
}

#[tonic::async_trait]
impl Pilot for PilotState {
    async fn get_command(&self, request: Request<GameState>) -> Result<Response<Command>, Status> {
        let mut cmd = Command {
            fire: false,
            rotation: Rotation::None as i32,
            thrusters: false,
        };

        let game_state = request.get_ref();
        let mut activity = self.activity.lock().unwrap();
        match *activity {
            Activity::Accelerate(counter) => {
                if counter == 0 {
                    *activity = Activity::Stop;
                } else {
                    *activity = Activity::Accelerate(counter - 1);
                    cmd.thrusters = true;
                }
            }
            Activity::Stop => {
                let speed = game_state.ship.as_ref()
                    .and_then(|s| s.velocity.as_ref())
                    .map_or(0.0, |v| v.speed());

                if speed < 0.5 {
                    let mut rng = rand::thread_rng();
                    let num_frames = rng.next_u32() % 100 + 100;
                    *activity = Activity::Accelerate(num_frames as usize);
                } else {
                    cmd = stop_ship(&game_state);
                }
            }
        }

        Ok(Response::new(cmd))
    }
}

fn stop_ship(game_state: &GameState) -> Command {
    let heading = Bearing::new(game_state.ship.as_ref().map_or(0.0, |s| s.heading));
    let course = Bearing::new(
        game_state
            .ship.as_ref()
            .and_then(|s| s.velocity.as_ref())
            .map_or(0.0, |v| v.bearing()),
    );
    let diff = heading.distance(&course);

    let mut cmd = Command {
        fire: false,
        rotation: Rotation::None as i32,
        thrusters: false,
    };

    // If we're not facing opposite to our motion, keep rotating to get there.
    if !diff.approx_eq_ratio(&PI, 0.01) {
        cmd.rotation = if diff.signum() as i8 > 0 {
            Rotation::Counterclockwise as i32
        } else {
            Rotation::Clockwise as i32
        };
    }
    // If we're still moving, fire thrusters
    else {
        cmd.thrusters = true;
    }

    cmd
}

// Note:
// Change in x under constant rotational velocity (r) and constant thrust (a) for time (t) and final heading h(t):
//    Vx(0) * t + a / 2 * (t^2 * sin(h(t)) + 2 * t * cos(h(t)) - 2 * sin(h(t)))

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // dotenv().ok();
    // env_logger::init();

    // info!("Starting simple-pilot");

    let addr = "[::1]:50051".parse().unwrap();
    // let addr = std::env::var("GRPC_SERVER_ADDRESS")?.parse()?;

    let pilot = PilotState {
        activity: Arc::new(Mutex::new(Activity::Stop)),
    };
    let svc = PilotServer::new(pilot);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
