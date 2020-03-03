use crate::components::{Transform, Velocity};
use nalgebra::{Isometry2, Vector2};
use specs::{Component, VecStorage};
use std::time::Duration;

pub struct Fragment {
    pub max_age: Duration,
    pub age: Duration,
}

impl Fragment {
    pub fn new() -> Self {
        Fragment {
            max_age: Duration::from_secs_f32(0.25),
            age: Duration::from_secs(0),
        }
    }

    pub fn radius() -> f32 {
        1.0
    }
    pub fn speed() -> f32 {
        500.0
    }
}

impl Component for Fragment {
    // TODO: Is this the wrong storage type? Use something sparser?
    type Storage = VecStorage<Self>;
}

pub fn make_fragment(x: f32, y: f32, bearing: f32) -> (Velocity, Transform, Fragment) {
    let transform = Transform(Isometry2::new(Vector2::<f32>::new(x, y), 0.0f32));

    (
        Velocity::from_speed_and_bearing(Fragment::speed(), bearing),
        transform,
        Fragment::new(),
    )
}
