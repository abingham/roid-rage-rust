use glam::Vec2;
use roid_rage_grpc::roid_rage as rpc;

use super::turn_or_thrust;

pub fn evade(ship: &rpc::Ship, roids: &[rpc::Roid]) -> rpc::Command {
    if roids.is_empty() {
        return rpc::Command::null();
    }

    let ship_pos = ship.position();
    let mut away_vector = Vec2::ZERO;
    let mut closest_delta = None;
    let mut closest_distance_sq = f32::INFINITY;

    for roid in roids {
        let delta = ship_pos - roid.position();
        let distance_sq = delta.length_squared();
        if distance_sq == 0.0 {
            continue;
        }

        if distance_sq < closest_distance_sq {
            closest_distance_sq = distance_sq;
            closest_delta = Some(delta);
        }

        away_vector += delta / distance_sq;
    }

    let target = if away_vector.length_squared() > 0.0 {
        away_vector
    } else {
        closest_delta.unwrap_or(Vec2::ZERO)
    };

    if target.length_squared() == 0.0 {
        return rpc::Command::null();
    }

    turn_or_thrust(ship.heading, target)
}

// #[cfg(test)] mod tests { use super::*;

//     mod stop {
//         use super::*;

//         #[test]
//         fn test_rotate_ccw() {
//             let ship = rpc::Ship {
//                 // facing 'east'
//                 heading: 0.0,
//                 mass: 1.0,
//                 position: None,
//                 thrust: 1.0,
//                 // moving 'south'
//                 velocity: Some(rpc::Velocity { x: 0.0, y: 1.0 }),
//                 cannon: None,
//             };
//             let cmd = stop(&ship);

//             assert_eq!(cmd.rotation, rpc::Rotation::Counterclockwise as i32);
//             assert!(!cmd.thrusters);
//             assert!(!cmd.fire);
//         }

//         #[test]
//         fn test_rotate_cw() {
//             let ship = rpc::Ship {
//                 // facing 'east'
//                 heading: 0.0,
//                 mass: 1.0,
//                 position: None,
//                 thrust: 1.0,
//                 // moving 'north'
//                 velocity: Some(rpc::Velocity { x: 0.0, y: -1.0 }),
//                 cannon: None,
//             };
//             let cmd = stop(&ship);

//             assert_eq!(cmd.rotation, rpc::Rotation::Clockwise as i32);
//             assert!(!cmd.thrusters);
//             assert!(!cmd.fire);
//         }

//         #[test]
//         fn test_fire_thrusters_when_facing_correct_direction() {
//             let ship = rpc::Ship {
//                 // facing 'east'
//                 heading: 0.0,
//                 mass: 1.0,
//                 position: None,
//                 thrust: 1.0,
//                 // moving 'west'
//                 velocity: Some(rpc::Velocity { x: -1.0, y: 0.0 }),
//                 cannon: None,
//             };
//             let cmd = stop(&ship);

//             assert_eq!(cmd.rotation, rpc::Rotation::None as i32);
//             assert!(cmd.thrusters);
//             assert!(!cmd.fire);
//         }

//         #[test]
//         fn test_do_nothing_when_already_stopped() {
//             let ship = rpc::Ship {
//                 // facing 'east'
//                 heading: 0.0,
//                 mass: 1.0,
//                 position: None,
//                 thrust: 1.0,
//                 // moving 'west'
//                 velocity: Some(rpc::Velocity { x: 0.0, y: 0.0 }),
//                 cannon: None,
//             };
//             let cmd = stop(&ship);

//             assert_eq!(cmd.rotation, rpc::Rotation::None as i32);
//             assert!(!cmd.thrusters);
//             assert!(!cmd.fire);
//         }
//     }
// }
