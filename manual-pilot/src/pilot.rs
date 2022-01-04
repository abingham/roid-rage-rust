use device_query::{DeviceQuery, DeviceState, Keycode};
use roid_rage_grpc::roid_rage as rpc;
use roid_rage_grpc::roid_rage::pilot_server::{Pilot};
use tonic::{Request, Response, Status};

pub struct PilotState {
    device_state: DeviceState,
}

impl PilotState {
    pub fn new() -> PilotState {
        PilotState {
            device_state: DeviceState::new(),
        }
    }
}

#[tonic::async_trait]
impl Pilot for PilotState {
    async fn get_command(
        &self,
        request: Request<rpc::GameState>,
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
                Keycode::S => {
                    let game_state = request.get_ref();
                    match &game_state.ship {
                        None => cmd,
                        Some(ship) => pilot_lib::steering::stop(ship)
                    }
                },
                Keycode::E => {
                    let game_state = request.get_ref();
                    match &game_state.ship {
                        None => cmd,
                        Some(ship) => pilot_lib::steering::evade(ship, &game_state.roids)
                    }
                }
                _ => cmd,
            }
        }

        Ok(Response::new(cmd))
    }
}

