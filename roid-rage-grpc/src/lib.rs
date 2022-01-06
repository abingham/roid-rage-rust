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
