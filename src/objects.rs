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

    pub fn update(&mut self, time_delta: f64) -> bool {
        self.position = self.position + self.velocity * time_delta;
        true
    }
}

pub struct Fragment {
    body: Circle,
    age: f64,
    max_age: f64,
}

impl Fragment {
    pub fn new(position: Point2<f64>, radius: f64, velocity: Vector2<f64>, max_age: f64) -> Fragment {
        Fragment {
            body: Circle::new(position, radius, velocity),
            age: 0.0,
            max_age: max_age
        }
    }

    pub fn id(&self) -> Uuid { self.body.id }

    pub fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) -> () {
        self.body.render(color, c, gl)
    }

    pub fn update(&mut self, time_delta: f64) -> bool {
        self.body.update(time_delta);
        self.age += time_delta;
        self.age <= self.max_age
    }
}