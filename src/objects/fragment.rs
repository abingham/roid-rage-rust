use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;
use uuid::Uuid;

use super::categories::Category;
use super::game_object::GameObject;

pub struct Fragment {
    collision_shape: Ball<f64>,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: Uuid,
    alive: bool,
    age: f64,
    max_age: f64,
}

impl Fragment {
    pub fn new(position: Point2<f64>, velocity: Vector2<f64>, max_age: f64) -> Fragment {
        Fragment {
            position: position,
            velocity: velocity,
            id: Uuid::new_v4(),
            alive: true,
            collision_shape: Ball::new(Fragment::radius()), // TODO: Can this be totally static?
            age: 0.0,
            max_age: max_age,
        }
    }

    pub fn radius() -> f64 {
        1.0
    }
}

impl GameObject for Fragment {
    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(
            -1.0 * Fragment::radius(),
            -1.0 * Fragment::radius(),
            2.0 * Fragment::radius(),
        );
        ellipse(*color, rect, transform, gl);
    }

    fn collision_shape(&self) -> &Shape<f64> {
        &self.collision_shape
    }

    fn update(&mut self, time_delta: f64) {
        self.set_position(self.position() + self.velocity() * time_delta);
        self.age += time_delta;
        if self.age > self.max_age {
            self.kill();
        }
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
