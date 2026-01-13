mod evade;
mod stop;
mod turn_to;

use glam::Vec2;
use roid_rage_grpc::roid_rage as rpc;

pub use evade::evade;
pub use stop::stop;
pub use turn_to::turn_to;

fn turn_or_thrust(heading: f32, target: Vec2) -> rpc::Command {
    let mut cmd = rpc::Command::null();
    let rotation = turn_to(heading, target);
    if rotation != rpc::Rotation::None {
        cmd.rotation = rotation as i32;
    } else {
        cmd.thrusters = true;
    }
    cmd
}
