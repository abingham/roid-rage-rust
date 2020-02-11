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

/// Calculate the speed represented by a velocity vector.
pub trait Velocity {
    fn speed(&self) -> f64;
    fn bearing(&self) -> f64;
}

impl Velocity for Vector2<f64> {
    fn speed(&self) -> f64 {
        (self[0].powf(2.0) + self[1].powf(2.0)).sqrt()
    }

    fn bearing(&self) -> f64 {
        self[1].atan2(self[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools_num::linspace;

    #[test]
    fn test_create_velocity_flat() {
        let vel = make_velocity_vector(10.0, 0.0);
        relative_eq!(vel[0], 10.0);
        relative_eq!(vel[1], 0.0);
    }

    #[test]
    fn test_create_velocity_up() {
        let vel = make_velocity_vector(10.0, PI / 2.0);
        relative_eq!(vel[0], 0.0);
        relative_eq!(vel[1], 10.0);
    }

    #[test]
    fn test_velocity_speed() {
        let speeds = vec![0.0, 9.2, 134.3, 900.6, 42.69];
        let bearings: Vec<f64> = linspace::<f64>(0.0, 2.0 * PI, 100).collect();
        for s in speeds {
            for b in &bearings {
                let vel = make_velocity_vector(s, *b);
                relative_eq!(vel.speed(), s);
            }
        }
    }

    #[test]
    fn test_velocity_bearing() {
        let speeds = vec![1.0]; // 0.001, 9.2, 134.3, 900.6, 42.69];
        let bearings: Vec<f64> = linspace::<f64>(-1.0 * PI, PI, 10).collect();
        for s in speeds {
            for b in &bearings {
                let vel = make_velocity_vector(s, *b);
                relative_eq!(vel.bearing(), *b);
            }
        }
    }
}
