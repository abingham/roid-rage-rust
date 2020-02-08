use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use uuid::Uuid;

use crate::game_object::GameObject;
use crate::field::Field;
use crate::object_set::ObjectSet;

pub struct Fragment {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: Uuid,
    age: f64,
    max_age: f64,
}

impl Fragment {
    pub fn new(position: Point2<f64>, velocity: Vector2<f64>, age: f64, max_age: f64) -> Fragment {
        Fragment {
            position: position,
            velocity: velocity,
            id: Uuid::new_v4(),
            age: age,
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

    fn update(&self, field: &Field, time_delta: f64) -> ObjectSet {
        let new_age = self.age + time_delta;

        if new_age > self.max_age {
            ObjectSet::new()
        }
        else {
            let new_position = field.wrap(&(self.position + self.velocity * time_delta));
            ObjectSet::from_objects(vec![], vec![], vec![Fragment::new(new_position, self.velocity, new_age, self.max_age)])
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
}
