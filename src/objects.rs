extern crate uuid;

use graphics;
use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use uuid::Uuid;


pub struct Circle {
    pub position: Point2<f64>,
    pub radius: f64,
    pub velocity: Vector2<f64>,
    id: Uuid
}

impl Circle {
    pub fn new(position: Point2<f64>, radius: f64, velocity: Vector2<f64>) -> Circle {
        Circle {
            position: position,
            radius: radius,
            velocity: velocity,
            id: Uuid::new_v4()
        }
    }

    pub fn id(&self) -> Uuid { self.id }

    pub fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) -> () {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);
    }

    pub fn update(&mut self, time_delta: f64) -> () {
        self.position = self.position + self.velocity * time_delta;
    }
}
