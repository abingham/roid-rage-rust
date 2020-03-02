use rand::prelude::*;
use std::f32::consts::PI;

pub fn random_bearing() -> f32 {
    let mut rng = thread_rng();
    (rng.gen::<f32>() * 2.0 - 1.0) * PI
}