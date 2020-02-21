use nalgebra::{Point2, Vector2};
use uuid::Uuid;

use super::super::traits::{Identifiable, Positioned};
use crate::model::field::Field;

pub struct Fragment {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: Uuid,
    age: f64,
    max_age: f64,
}

impl Fragment {
    pub fn new(position: Point2<f64>, velocity: Vector2<f64>, age: f64, max_age: f64) -> Fragment {
        Fragment {
            position: position,
            velocity: velocity,
            id: Uuid::new_v4(),
            age: age,
            max_age: max_age,
        }
    }

    pub fn radius() -> f64 {
        1.0
    }
}

impl Positioned for Fragment {
    fn project(&mut self, _field: &Field, time_delta: f64) -> () {
        self.age += time_delta;
        self.position += self.velocity * time_delta;
    }

    fn position(&self) -> Point2<f64> {
        self.position
    }
}

impl Identifiable for Fragment {
    fn id(&self) -> Uuid {
        self.id
    }

    fn alive(&self) -> bool {
        self.age <= self.max_age
    }
}
