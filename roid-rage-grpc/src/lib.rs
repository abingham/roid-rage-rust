pub mod roid_rage {
    tonic::include_proto!("roid_rage");

    /// Allow our grpc Velocity to be used as a sted::Velocity.
    // impl sted::Velocity<f32> for Velocity {
    //     fn dx(&self) -> f32 {
    //         self.x
    //     }

    //     fn dy(&self) -> f32 {
    //         self.y
    //     }
    // }

    // impl sted::Direction<f32> for Direction {
    //     fn dx(&self) -> f32 { self.x }
    //     fn dy(&self) -> f32 { self.y }
    
    //     fn create(x: f32, y: f32) -> Direction { Direction {x: x, y: y} }
    // }

    impl Roid {
        /// Get a velocity from a roid, producing a default velocity if necessary.
        pub fn velocity(&self) -> glam::Vec2 {
            match &self.velocity {
                None => glam::Vec2::ZERO,
                Some(v) => glam::Vec2::from(v),
            }
        }

        pub fn position(&self) -> glam::Vec2 {
            match &self.position {
                None => glam::Vec2::ZERO,
                Some(p) => glam::Vec2::from(p),
            }
        }
    }

    impl Ship {
        // Get a velocity from a ship, producing a default velocity if necessary.
        pub fn velocity(&self) -> glam::Vec2 {
            match &self.velocity {
                None => glam::Vec2::ZERO,
                Some(v) => glam::Vec2::from(v),
            }
        }

        pub fn position(&self) -> glam::Vec2 {
            match &self.position {
                None => glam::Vec2::ZERO,
                Some(p) => glam::Vec2::from(p),
            }
        }
    }

    impl Command {
        /// Build a Command that does nothing.
        pub fn null() -> Command {
            Command {
                fire: false,
                rotation: Rotation::None as i32,
                thrusters: false,
            }
        }
    }

    // Conversions from RPC types to glam

    impl From<&Position> for glam::Vec2 {
        fn from(p: &Position) -> glam::Vec2 {
            glam::Vec2::new(p.x, p.y)
        }
    }

    impl From<&Velocity> for glam::Vec2 {
        fn from(p: &Velocity) -> glam::Vec2 {
            glam::Vec2::new(p.x, p.y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::roid_rage::{Command, Position, Roid, Ship, Velocity};

    #[test]
    fn ship_velocity_defaults_to_zero() {
        let ship = Ship {
            velocity: None,
            ..Default::default()
        };
        let v = ship.velocity();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    fn ship_position_defaults_to_zero() {
        let ship = Ship {
            position: None,
            ..Default::default()
        };
        let p = ship.position();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn roid_position_defaults_to_zero() {
        let roid = Roid {
            position: None,
            ..Default::default()
        };
        let p = roid.position();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn roid_velocity_defaults_to_zero() {
        let roid = Roid {
            velocity: None,
            ..Default::default()
        };
        let v = roid.velocity();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    fn command_null_is_noop() {
        let cmd = Command::null();
        assert!(!cmd.fire);
        assert!(!cmd.thrusters);
        assert_eq!(cmd.rotation, super::roid_rage::Rotation::None as i32);
    }

    #[test]
    fn from_position_and_velocity() {
        let p = Position { x: 3.0, y: -2.0 };
        let v = Velocity { x: -1.0, y: 4.0 };
        let gp: glam::Vec2 = (&p).into();
        let gv: glam::Vec2 = (&v).into();
        assert_eq!(gp.x, 3.0);
        assert_eq!(gp.y, -2.0);
        assert_eq!(gv.x, -1.0);
        assert_eq!(gv.y, 4.0);
    }
}
