use graphics;
use nalgebra::Point2;
use ncollide2d::shape::ShapeHandle;
use opengl_graphics::GlGraphics;
use std::hash::{Hash, Hasher};
use uuid;
use crate::velocity;
use nalgebra::Vector2;

use super::field::Field;

#[derive(Eq, PartialEq)]
pub enum Kind {
    Weapon,
    Ship,
    Roid,
    Debris
}

pub trait GameObject {
    fn position(&self) -> &Point2<f64>;

    fn project(&mut self, field: &Field, time_delta: f64) -> ();

    // TODO: This should be in a view
    fn render(&self, c: graphics::Context, gl: &mut GlGraphics);

    fn id(&self) -> uuid::Uuid;

    fn alive(&self) -> bool {
        true
    }

    fn explode(&self) -> Vec<Box<dyn GameObject>> {
        vec![]
    }

    fn kind(&self) -> Kind;

    fn collision_shape(&self) -> ShapeHandle<f64>;
}

impl PartialEq for dyn GameObject {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for dyn GameObject {}

impl Hash for dyn GameObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
