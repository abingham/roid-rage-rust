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

use roid_rage_grpc::roid_rage::{Command, GameState, Rotation, Ship};

/// Activity state for the pilot, i.e. what is it "doing"
enum Activity {
    Accelerate(usize),
    Stop,
}

struct PilotState {
    activity: Arc<Mutex<Activity>>,
}

impl PilotState {
    /// Update internal state based on game state and calculate a 
    /// new Command.
    fn update(&self, ship: &Ship) -> Command {
        let mut cmd = Command::null();
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
                let speed = ship.velocity().speed();

                if speed < 0.5 {
                    let mut rng = rand::thread_rng();
                    let num_frames = rng.next_u32() % 100 + 100;
                    *activity = Activity::Accelerate(num_frames as usize);
                } else {
                    cmd = PilotState::stop_ship(ship);
                }
            }
        }

        cmd
    }

    /// Take action to stop the ship.
    fn stop_ship(ship: &Ship) -> Command {
        let heading = Bearing::new(ship.heading);
        let course = Bearing::new(ship.velocity().bearing());
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
}

#[tonic::async_trait]
impl Pilot for PilotState {
    async fn get_command(&self, request: Request<GameState>) -> Result<Response<Command>, Status> {
        let game_state = request.get_ref();

        let cmd = match &game_state.ship {
            None => Command::null(),
            Some(ship) => self.update(&ship),
        };

        Ok(Response::new(cmd))
    }
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
