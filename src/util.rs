use nalgebra::Vector2;
use std::f64;

pub fn make_velocity_vector(speed: f64, bearing: f64) -> Vector2<f64> {
    Vector2::new(
        bearing.cos() * speed,
        bearing.sin() * speed
    )
}