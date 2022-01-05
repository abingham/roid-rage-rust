use glam::Vec2;

pub mod roid_rage {
    tonic::include_proto!("roid_rage");

    /// Allow our grpc Velocity to be used as a sted::Velocity.
    // impl sted::Velocity<f32> for Vector {
    //     fn dx(&self) -> f32 {
    //         self.x
    //     }

    //     fn dy(&self) -> f32 {
    //         self.y
    //     }
    // }

    impl Roid {
        /// Get a velocity from a roid, producing a default velocity if necessary.
        pub fn velocity(&self) -> glam::Vec2 {
            match &self.velocity {
                None => glam::Vec2::ZERO,
                Some(v) => glam::Vec2::new(v.x, v.y),
            }
        }

        // pub fn position(&self) -> &Position {
        //     match &self.position {
        //         None => &Position { x: 0.0, y: 0.0 },
        //         Some(p) => p,
        //     }
        // }
    }

    impl Ship {
        // Get a velocity from a ship, producing a default velocity if necessary.
        // pub fn velocity(&self) -> &Velocity {
        //     match &self.velocity {
        //         None => &Velocity { x: 0.0, y: 0.0 },
        //         Some(v) => v,
        //     }
        // }

        // pub fn position(&self) -> &Position {
        //     match &self.position {
        //         None => &Position { x: 0.0, y: 0.0 },
        //         Some(p) => p,
        //     }
        // }
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

    // impl From<&Position> for glam::Vec2 {
    //     fn from(p: &Position) -> glam::Vec2 {
    //         glam::Vec2::new(p.x, p.y)
    //     }
    // }
}
