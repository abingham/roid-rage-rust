use nalgebra::Vector2;
use rand::prelude::*;
use std::f64;
use std::f64::consts::PI;

pub fn random_bearing() -> f64 {
    let mut rng = thread_rng();
    (rng.gen::<f64>() * 2.0 - 1.0) * PI
}

pub fn make_velocity_vector(speed: f64, bearing: f64) -> Vector2<f64> {
    Vector2::new(bearing.cos() * speed, bearing.sin() * speed)
}
