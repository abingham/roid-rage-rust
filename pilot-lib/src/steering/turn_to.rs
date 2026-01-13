use float_cmp::ApproxEqRatio;
use glam::Vec2;
use roid_rage_grpc::roid_rage as rpc;
use sted::to_vector;

pub fn turn_to(heading: f32, target: Vec2) -> rpc::Rotation {
    if target.length_squared() == 0.0 {
        return rpc::Rotation::None;
    }

    let diff = to_vector(heading).angle_to(target);

    if diff.approx_eq_ratio(&0.0, 0.01) {
        rpc::Rotation::None
    } else if diff.signum() as i8 > 0 {
        rpc::Rotation::Counterclockwise
    } else {
        rpc::Rotation::Clockwise
    }
}
