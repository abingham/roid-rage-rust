use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;

use crate::collide::Collidable;
use crate::field::Field;
use crate::game_object::GameObject;
use crate::object_set::ObjectSet;

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
            collision_shape: Ball::new(Bullet::radius()),
            id: uuid::Uuid::new_v4()
        }
    }

    pub fn radius() -> f64 {
        2.0
    }

    pub fn speed() -> f64 {
        400.0
    }
}

impl GameObject for Bullet {
    fn id(&self) -> uuid::Uuid { self.id }

    fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }

    fn update(&self, field: &Field, time_delta: f64) -> ObjectSet {
        let new_position = self.position + self.velocity * time_delta;
        if !field.contains(&new_position) {
            ObjectSet::new()
        }
        else {
            ObjectSet::from_objects(
                vec![], 
                vec![Bullet::new(new_position, self.velocity.clone())], 
                vec![])
        }
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