use graphics;
use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use uuid;
use std::hash::{Hash, Hasher};

use crate::object_set::ObjectSet;

use crate::field::Field;

pub trait GameObject {
    fn position(&self) -> &Point2<f64>;

    fn velocity(&self) -> &Vector2<f64>;

    fn update(&mut self, field: &Field, time_delta: f64);

    fn render(&self, 
              color: &[f32; 4], 
              c: graphics::Context, 
              gl: &mut GlGraphics);

    fn id(&self) -> uuid::Uuid;

    fn alive(&self) -> bool;

    fn kill(&mut self) -> ObjectSet;
}

impl PartialEq for GameObject {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for GameObject {}

impl Hash for GameObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

