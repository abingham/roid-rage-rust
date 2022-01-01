pub mod roid_rage {
    tonic::include_proto!("roid_rage");

    /// Allow our grpc Velocity to be used as a sted::Velocity.
    impl sted::Velocity<f32> for Velocity {
        fn dx(&self) -> f32 {
            self.x
        }

        fn dy(&self) -> f32 {
            self.y
        }
    }

    impl Ship {
        /// Get a velocity from a ship, producing a default velocity if necessary.
        pub fn velocity(&self) -> &Velocity {
            match &self.velocity {
                None => &Velocity { x: 0.0, y: 0.0 },
                Some(v) => v,
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
}
