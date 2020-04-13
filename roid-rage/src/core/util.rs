use rand::prelude::*;
use std::f32::consts::PI;
use nalgebra::Vector2;

pub fn random_bearing() -> f32 {
    let mut rng = thread_rng();
    (rng.gen::<f32>() * 2.0 - 1.0) * PI
}

/// Create a velocity vector from speed and bearing.
pub fn from_speed_and_bearing(speed: f32, bearing: f32) -> Vector2<f32> {
    Vector2::<f32>::new(bearing.cos(), bearing.sin()) * speed
}
