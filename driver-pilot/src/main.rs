extern crate nalgebra;

use rand::Rng;
use std::sync::Arc;
use std::sync::Mutex;
use sted::Velocity;
use tonic::{Request, Response, Status};

use roid_rage_grpc::roid_rage::pilot_server::Pilot;

use roid_rage_grpc::roid_rage::{Command, GameState, Ship};

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
                    let mut rng = rand::rng();
                    let num_frames = rng.random::<u32>() % 100 + 100;
                    *activity = Activity::Accelerate(num_frames as usize);
                } else {
                    cmd = pilot_lib::steering::stop(ship);
                }
            }
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
    let pilot = PilotState {
        activity: Arc::new(Mutex::new(Activity::Stop)),
    };

    pilot_lib::pilot_base::pilot_main(pilot).await
}
