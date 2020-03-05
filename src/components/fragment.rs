use crate::components::{Transform, LinearMotion};
use nalgebra::{Isometry2, Vector2};
use specs::{Component, VecStorage};
use std::time::Duration;
use rand::prelude::*;
use crate::types::velocity::from_speed_and_bearing;

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
}

impl Component for Fragment {
    // TODO: Is this the wrong storage type? Use something sparser?
    type Storage = VecStorage<Self>;
}

// TODO: This should use the builder pattern from e.g. roid.
pub fn make_fragment(x: f32, y: f32, bearing: f32) -> (LinearMotion, Transform, Fragment) {
    let transform = Transform(Isometry2::new(Vector2::<f32>::new(x, y), 0.0f32));
    let speed = thread_rng().gen::<f32>() * 250.0 + 250.0;
    (
        LinearMotion(from_speed_and_bearing(speed, bearing)),
        transform,
        Fragment::new(),
    )
}
