use float_cmp::ApproxEqRatio;
use glam::Vec2;
// use roid_rage::core::collide::collision_vector;
use roid_rage_grpc::roid_rage as rpc;
use std::f32::consts::PI;
use sted::{Direction};

// TODO: This is super half-assed right now.
pub fn evade(ship: &rpc::Ship, roids: &Vec<rpc::Roid>) -> rpc::Command {
    let mut cmd = rpc::Command {
        fire: false,
        rotation: rpc::Rotation::None as i32,
        thrusters: false,
    };

    // Turn away from all roids
    let pressure_vector = roids
        .iter()
        .map(|roid| Vec2::from(roid.position()) - Vec2::from(ship.position()))
        .fold(Vec2::new(0.0, 0.0), |acc, x| acc + x);

    let diff = pressure_vector.dot(ship.heading.vector());

    // TODO: We use this same code in 'stop()'. Perhaps it should be
    // factored into some sort of "turn_to()" function.
    if !diff.approx_eq_ratio(&PI, 0.01) {
        cmd.rotation = if diff.signum() as i8 > 0 {
            rpc::Rotation::Counterclockwise as i32
        } else {
            rpc::Rotation::Clockwise as i32
        };
    }
    // If we're still moving, fire thrusters
    else {
        cmd.thrusters = true;
    }

    cmd
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
