use nalgebra::{Point2, Vector2};

use crate::model::field::Field;
use super::super::traits::{Identifiable, Positioned};
use rand::prelude::*;
use uuid;

pub struct Roid {
    radius: f64,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: uuid::Uuid,
    color: [f32; 4],
}

impl Roid {
    pub fn new(position: Point2<f64>, radius: f64, velocity: Vector2<f64>) -> Roid {
        let mut rng = thread_rng();
        let color: f32 = rng.gen::<f32>() * 0.5 + 0.5;
        Roid {
            position: position,
            velocity: velocity,
            radius: radius,
            id: uuid::Uuid::new_v4(),
            color: [color, color, color, 1.0],
        }
    }

    pub fn min_radius() -> f64 { 10.0 }

    pub fn color(&self) -> &[f32;4] {
        &self.color
    }

    pub fn radius(&self) -> f64 {
         self.radius
    }

    pub fn velocity(&self) -> Vector2<f64> {
        self.velocity
    }
}

impl Positioned for Roid {
    fn position(&self) -> Point2<f64> {
        self.position
    }

    fn project(&mut self, field: &Field, time_delta: f64) -> () {
        self.position += self.velocity * time_delta;
        if !field.contains(&self.position) {
            self.position = field.wrap(&self.position);
        }
    }
}

impl Identifiable for Roid {
    fn id(&self) -> uuid::Uuid {
        self.id
    }
}
