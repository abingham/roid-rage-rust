use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;

use crate::collide::Collidable;
use crate::field::Field;
use crate::traits::{Moving, Renderable, Updateable};
use crate::util::project;
use std::hash::{Hash, Hasher};

// TODO: Remove Uuid library from package if no longer used.

pub struct Bullet {
    collision_shape: Ball<f64>,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: uuid::Uuid
}

impl Bullet {
    pub fn new(position: Point2<f64>, velocity: Vector2<f64>) -> Bullet {
        Bullet {
            position: position,
            velocity: velocity,
            collision_shape: Ball::new(Bullet::radius()), // TODO: Can this be totally static?
            id: uuid::Uuid::new_v4()
        }
    }

    pub fn radius() -> f64 {
        2.0
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
}

impl PartialEq for Bullet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Bullet {}

impl Hash for Bullet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Moving for Bullet {
    fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }
}

impl Updateable for Bullet {
    fn update(&mut self, field: &Field, time_delta: f64) {
        self.position = project(self, time_delta);
    }
}

impl Collidable for Bullet {
    fn collision_shape(&self) -> &dyn Shape<f64> {
        &self.collision_shape
    }
}

impl Renderable for Bullet {
    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(
            -1.0 * Bullet::radius(),
            -1.0 * Bullet::radius(),
            2.0 * Bullet::radius(),
        );
        ellipse(*color, rect, transform, gl);
    }
}
