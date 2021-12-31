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
}
