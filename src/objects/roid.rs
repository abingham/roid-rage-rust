use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;
use uuid::Uuid;

use super::traits::{Collidable, Identifiable, Mortal, Positioned, Renderable};

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

impl Renderable for Roid {
    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);
    }
}

impl Collidable for Roid {
    fn collision_shape(&self) -> &Shape<f64> {
        &self.collision_shape
    }
}

impl Positioned for Roid {
    fn position(&self) -> &Point2<f64> {
        &self.position
    }
    fn set_position(&mut self, pos: Point2<f64>) {
        self.position = pos;
    }
    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }
}

impl Identifiable for Roid {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Mortal for Roid {
    fn alive(&self) -> bool {
        self.alive
    }
    fn kill(&mut self) {
        self.alive = false;
    }
}