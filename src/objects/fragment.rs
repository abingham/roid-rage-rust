use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;
use uuid::Uuid;

use crate::game_object::GameObject;
use crate::field::Field;
use crate::util::project;
use crate::object_set::ObjectSet;

pub struct Fragment {
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

    fn update(&mut self, field: &Field, time_delta: f64) {
        self.position = field.wrap(&project(self, time_delta));

        self.age += time_delta;
        if self.age > self.max_age {
            self.kill();
        }
    }
    fn position(&self) -> &Point2<f64> {
        &self.position
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

    fn kill(&mut self) -> ObjectSet {
        self.alive = false;
        ObjectSet::new()
    }
}
