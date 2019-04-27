use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;
use uuid::Uuid;

use super::categories::Category;
use super::game_object::GameObject;

pub struct Roid {
    radius: f64,
    collision_shape: Ball<f64>,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: Uuid,
    alive: bool,
}

impl Roid {
    pub fn new(position: Point2<f64>, radius: f64, velocity: Vector2<f64>) -> Roid {
        Roid {
            position: position,
            velocity: velocity,
            id: Uuid::new_v4(),
            alive: true,
            radius: radius,
            collision_shape: Ball::new(radius),
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl GameObject for Roid {
    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);
    }

    fn collision_shape(&self) -> &Shape<f64> {
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
    fn kill(&mut self) -> Vec<(Category, Box<GameObject>)> {
        self.alive = false;
        vec![]
    }
}
