use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;

use crate::collide::Collidable;
use crate::field::Field;
use crate::game_object::GameObject;
use crate::util::project;
use crate::object_set::ObjectSet;
use std::hash::{Hash, Hasher};

pub struct Bullet {
    collision_shape: Ball<f64>,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: uuid::Uuid,
    alive: bool
}

impl Bullet {
    pub fn new(position: Point2<f64>, velocity: Vector2<f64>) -> Bullet {
        Bullet {
            position: position,
            velocity: velocity,
            collision_shape: Ball::new(Bullet::radius()),
            id: uuid::Uuid::new_v4(),
            alive: true
        }
    }

    pub fn radius() -> f64 {
        2.0
    }
}

impl GameObject for Bullet {
    fn id(&self) -> uuid::Uuid { self.id }
    fn alive(&self) -> bool { self.alive }
    fn kill(&mut self) -> ObjectSet { 
        self.alive = false; 
        ObjectSet::new()
    }

   fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }

    fn update(&mut self, field: &Field, time_delta: f64) {
        self.position = project(self, time_delta);
    }

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

impl Collidable for Bullet {
    fn collision_shape(&self) -> &dyn Shape<f64> {
        &self.collision_shape
    }

    fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }
}