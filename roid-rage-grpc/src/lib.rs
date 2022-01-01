pub mod roid_rage {
    tonic::include_proto!("roid_rage");

    impl sted::Velocity<f32> for Velocity {
        fn dx(&self) -> f32 {
            self.x
        }

        fn dy(&self) -> f32 {
            self.y
        }
    }

    impl Ship {
        pub fn velocity(&self) -> &Velocity {
            match &self.velocity {
                None => &Velocity { x: 0.0, y: 0.0},
                Some(v) => v
            }
        }
    }
}
