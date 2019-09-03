use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;
use uuid::Uuid;

use super::categories::Category;
use super::game_object::GameObject;

pub struct Bullet {
    collision_shape: Ball<f64>,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: Uuid,
    alive: bool,
}

impl Bullet {
    pub fn new(position: Point2<f64>, velocity: Vector2<f64>) -> Bullet {
        Bullet {
            position: position,
            velocity: velocity,
            id: Uuid::new_v4(),
            alive: true,
            collision_shape: Ball::new(Bullet::radius()), // TODO: Can this be totally static?
        }
    }

    pub fn radius() -> f64 {
        2.0
    }
}

impl GameObject for Bullet {
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

    fn collision_shape(&self) -> &dyn Shape<f64> {
        &self.collision_shape
    }

    fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn set_position(&mut self, pos: Point2<f64>) {
        self.position = pos;
    }

    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) -> Vec<(Category, Box<dyn GameObject>)> {
        self.alive = false;
        vec![]
    }
}
