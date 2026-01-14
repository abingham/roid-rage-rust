use float_cmp::ApproxEqRatio;
use glam::Vec2;
use roid_rage_grpc::roid_rage as rpc;
use sted::to_vector;

pub fn turn_to(heading: f32, target: Vec2) -> rpc::Rotation {
    if target.length_squared() == 0.0 {
        return rpc::Rotation::None;
    }

    let target_dir = target.normalize_or_zero();
    if target_dir.length_squared() == 0.0 {
        return rpc::Rotation::None;
    }

    let diff = to_vector(heading).angle_to(target_dir);

    if diff.approx_eq_ratio(&0.0, 0.01) {
        rpc::Rotation::None
    } else if diff.signum() as i8 > 0 {
        rpc::Rotation::Clockwise
    } else {
        rpc::Rotation::Counterclockwise
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sted::to_vector;

    #[test]
    fn returns_none_for_zero_target() {
        assert_eq!(turn_to(0.0, Vec2::ZERO), rpc::Rotation::None);
    }

    #[test]
    fn returns_none_when_aligned() {
        let heading = 0.0;
        let target = to_vector(heading);
        assert_eq!(turn_to(heading, target), rpc::Rotation::None);
    }

    #[test]
    fn chooses_rotation_for_off_axis_target() {
        let heading = 0.0;
        let target = Vec2::new(0.0, 1.0);
        assert_eq!(turn_to(heading, target), rpc::Rotation::Clockwise);
    }
}
