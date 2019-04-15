extern crate uuid;

use graphics;
use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use uuid::Uuid;
use ncollide2d::shape::{Ball, Shape};

pub struct Core {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: Uuid,
    alive: bool,
}

pub trait GameObject {
    fn get_core(&self) -> &Core;
    fn get_mut_core(&mut self) -> &mut Core;

    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics);
    fn update(&mut self, time_delta: f64);
    fn collision_shape(&self) -> &Shape<f64>;

    fn position(&self) -> &Point2<f64> { &self.get_core().position }
    fn set_position(&mut self, pos: Point2<f64>) { self.get_mut_core().position = pos; }
    fn velocity(&self) -> &Vector2<f64> { &self.get_core().velocity }
    fn id(&self) -> Uuid { self.get_core().id }
    fn alive(&self) -> bool { self.get_core().alive }
    fn kill(&mut self) { self.get_mut_core().alive = false; }
}

pub struct Circle {
    core: Core,
    radius: f64,
    collision_shape: Ball<f64>,
}

impl Circle {
    pub fn new(position: Point2<f64>, radius: f64, velocity: Vector2<f64>) -> Circle {
        Circle {
            core: Core {
                position: position,
                velocity: velocity,
                id: Uuid::new_v4(),
                alive: true
            },
            radius: radius,
            collision_shape: Ball::new(radius),
        }
    }

    pub fn radius(&self) -> f64 { self.radius }

}

impl GameObject for Circle {
    fn get_core(&self) -> &Core { &self.core }
    fn get_mut_core(&mut self) -> &mut Core { &mut self.core }

    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.core.position.coords[0], self.core.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);

    }
    fn update(&mut self, time_delta: f64) {
        self.core.position = self.core.position + self.core.velocity * time_delta;
    }
    fn collision_shape(&self) -> &Shape<f64> {
        &self.collision_shape
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
}

impl GameObject for Fragment {
    fn get_core(&self) -> &Core { self.body.get_core() }

    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) -> () {
        self.body.render(color, c, gl)
    }

    fn update(&mut self, time_delta: f64) {
        self.body.update(time_delta);
        self.age += time_delta;
        if self.age > self.max_age {
            self.kill();
        }
    }

    fn set_position(&mut self, pos: Point2<f64>) { self.body.set_position(pos) }

    fn collision_shape(&self) -> &Shape<f64> { self.body.collision_shape() }
}