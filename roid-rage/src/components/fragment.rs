use crate::components::{LinearVelocity, Position};
use crate::core::velocity::from_speed_and_bearing;
use nalgebra::Vector2;
use rand::prelude::*;
use specs::{Component, HashMapStorage};
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
}

impl Component for Fragment {
    type Storage = HashMapStorage<Self>;
}

// TODO: This should use the builder pattern from e.g. roid.
pub fn make_fragment(x: f32, y: f32, bearing: f32) -> (LinearVelocity, Position, Fragment) {
    let speed = thread_rng().gen::<f32>() * 250.0 + 250.0;
    (
        LinearVelocity(from_speed_and_bearing(speed, bearing)),
        Position(Vector2::<f32>::new(x, y)),
        Fragment::new(),
    )
}
