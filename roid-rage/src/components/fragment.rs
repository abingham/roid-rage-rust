use crate::components::{LinearVelocity, Position};
use crate::core::util::from_quantity_and_bearing;
use glam::Vec2;
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

pub fn make_fragment<B>(builder: B, x: f32, y: f32, course: f32)
where
    B: specs::world::Builder,
{
    let speed = thread_rng().gen::<f32>() * 250.0 + 250.0;

    builder
        .with(LinearVelocity(from_quantity_and_bearing(speed, course)))
        .with(Position(Vec2::new(x, y)))
        .with(Fragment::new())
        .build();
}
