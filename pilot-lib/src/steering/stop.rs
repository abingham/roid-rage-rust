use roid_rage_grpc::roid_rage as rpc;
use sted::Velocity;

use super::turn_or_thrust;

/// Bring the ship to a stop
///
/// Rotate until we're facing the opposite direction of the
/// ship's heading, and then fire thrusters until speed
/// is zero.
pub fn stop(ship: &rpc::Ship) -> rpc::Command {
    let velocity = ship.velocity();

    // If already stopped, do nothing
    if velocity.speed() == 0.0 {
        return rpc::Command::null();
    }

    turn_or_thrust(ship.heading, -velocity)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod stop {
        use super::*;

        #[test]
        fn test_rotate_ccw() {
            let ship = rpc::Ship {
                // facing 'east'
                heading: 0.0,
                mass: 1.0,
                position: None,
                thrust: 1.0,
                // moving 'south'
                velocity: Some(rpc::Velocity { x: 0.0, y: 1.0 }),
                cannon: None,
            };
            let cmd = stop(&ship);

            assert_eq!(cmd.rotation, rpc::Rotation::Counterclockwise as i32);
            assert!(!cmd.thrusters);
            assert!(!cmd.fire);
        }

        #[test]
        fn test_rotate_cw() {
            let ship = rpc::Ship {
                // facing 'east'
                heading: 0.0,
                mass: 1.0,
                position: None,
                thrust: 1.0,
                // moving 'north'
                velocity: Some(rpc::Velocity { x: 0.0, y: -1.0 }),
                cannon: None,
            };
            let cmd = stop(&ship);

            assert_eq!(cmd.rotation, rpc::Rotation::Clockwise as i32);
            assert!(!cmd.thrusters);
            assert!(!cmd.fire);
        }

        #[test]
        fn test_fire_thrusters_when_facing_correct_direction() {
            let ship = rpc::Ship {
                // facing 'east'
                heading: 0.0,
                mass: 1.0,
                position: None,
                thrust: 1.0,
                // moving 'west'
                velocity: Some(rpc::Velocity { x: -1.0, y: 0.0 }),
                cannon: None,
            };
            let cmd = stop(&ship);

            assert_eq!(cmd.rotation, rpc::Rotation::None as i32);
            assert!(cmd.thrusters);
            assert!(!cmd.fire);
        }

        #[test]
        fn test_do_nothing_when_already_stopped() {
            let ship = rpc::Ship {
                // facing 'east'
                heading: 0.0,
                mass: 1.0,
                position: None,
                thrust: 1.0,
                // moving 'west'
                velocity: Some(rpc::Velocity { x: 0.0, y: 0.0 }),
                cannon: None,
            };
            let cmd = stop(&ship);

            assert_eq!(cmd.rotation, rpc::Rotation::None as i32);
            assert!(!cmd.thrusters);
            assert!(!cmd.fire);
        }
    }
}
