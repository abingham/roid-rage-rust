use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use ncollide2d::shape::{Ball, ShapeHandle};
use uuid::Uuid;

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

    pub fn project(&mut self, _field: &Field, time_delta: f64) -> () {
        self.age += time_delta;
        self.position += self.velocity * time_delta;
    }

    pub fn position(&self) -> &Point2<f64> {
        &self.position
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn alive(&self) -> bool {
        self.age <= self.max_age
    }

    pub fn collision_shape(&self) -> ShapeHandle<f64> {
        ShapeHandle::new(Ball::new(Fragment::radius()))
    }
}
