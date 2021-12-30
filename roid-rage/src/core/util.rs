use glam::Vec2;
use rand::prelude::*;
use std::f32::consts::PI;

pub fn random_bearing() -> f32 {
    let mut rng = thread_rng();
    (rng.gen::<f32>() * 2.0 - 1.0) * PI
}

/// Create a vector from quantity (e.g. speed) and bearing.
pub fn from_quantity_and_bearing(quantity: f32, bearing: f32) -> Vec2 {
    Vec2::new(bearing.cos(), bearing.sin()) * quantity
}
