use crate::game_object::GameObject;
use nalgebra::{Point2, Vector2};
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

/// Project the position of a moving object forward in time.
pub fn project(m: &dyn GameObject, time_delta: f64) -> Point2<f64> {
    m.position() + m.velocity() * time_delta
}

/// Calculate the speed represented by a velocity vector.
pub fn speed(velocity: &Vector2<f64>) -> f64 {
    (velocity[0].powf(2.0) + velocity[1].powf(2.0)).sqrt()
}

pub fn bearing(velocity: &Vector2<f64>) -> f64 {
    (velocity[1] / velocity[0]).atan()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools_num::linspace;

    const EPSILON: f64 = 0.000000001;

    macro_rules! assert_f64_eq {
        ( $( $x:expr, $y:expr ),* ) => {
            {
                $(
                assert!(($x - $y).abs() <= EPSILON);
                )*
            }
        };
    }

    #[test]
    fn test_create_velocity_flat() {
        let vel = make_velocity_vector(10.0, 0.0);
        assert_f64_eq!(vel[0], 10.0);
        assert_f64_eq!(vel[1], 0.0);
    }

    #[test]
    fn test_create_velocity_up() {
        let vel = make_velocity_vector(10.0, PI / 2.0);
        assert_f64_eq!(vel[0], 0.0);
        assert_f64_eq!(vel[1], 10.0);
    }

    #[test]
    fn test_velocity_speed() {
        let speeds = vec![0.0, 9.2, 134.3, 900.6, 42.69];
        let bearings: Vec<f64> = linspace::<f64>(0.0, 2.0 * PI, 100).collect();
        for s in speeds {
            for b in &bearings {
                let vel = make_velocity_vector(s, *b);
                assert_f64_eq!(speed(&vel), s);
            }
        }
    }

    #[test]
    fn test_velocity_bearing() {
        let speeds = vec![0.001, 9.2, 134.3, 900.6, 42.69];
        let bearings: Vec<f64> = linspace::<f64>(1.0, 2.0 * PI, 100).collect();
        for s in speeds {
            for b in &bearings {
                let vel = make_velocity_vector(s, 1.0);
                assert_f64_eq!(bearing(&vel), 1.0);
            }
        }
    }
}
