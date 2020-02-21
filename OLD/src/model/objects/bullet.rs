use nalgebra::{Point2, Vector2};

use super::super::traits::{Identifiable, Positioned};
use crate::model::field::Field;
use crate::velocity::make_velocity_vector;

pub struct Bullet {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: uuid::Uuid,
}

impl Bullet {
    pub fn new(position: Point2<f64>, bearing: f64) -> Bullet {
        Bullet {
            position: position,
            velocity: make_velocity_vector(Bullet::speed(), bearing),
            id: uuid::Uuid::new_v4(),
        }
    }

    pub fn radius() -> f64 {
        2.0
    }

    pub fn speed() -> f64 {
        600.0
    }
}

impl Positioned for Bullet {
    fn position(&self) -> Point2<f64> {
        self.position
    }

    fn project(&mut self, _field: &Field, time_delta: f64) -> () {
        let new_position = self.position + self.velocity * time_delta;
        self.position = new_position;
    }
}

impl Identifiable for Bullet {
    fn id(&self) -> uuid::Uuid {
        self.id
    }
}
