use graphics;
use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use uuid;
use std::hash::{Hash, Hasher};
use ncollide2d::shape::ShapeHandle;
use ncollide2d::pipeline::CollisionGroups;

use crate::object_set::ObjectSet;

use crate::field::Field;

// Collision groups
pub const MASSIVE_GROUP: usize = 1;
pub const WEAPON_GROUP: usize = 2;
pub const SHIP_GROUP: usize = 3;

pub trait GameObject {
    fn position(&self) -> &Point2<f64>;

    fn velocity(&self) -> &Vector2<f64>;

    fn update(&mut self, field: &Field, time_delta: f64) -> ();

    fn render(&self, 
              color: &[f32; 4], 
              c: graphics::Context, 
              gl: &mut GlGraphics);

    fn id(&self) -> uuid::Uuid;

    fn explode(&self) -> ObjectSet {
        ObjectSet::new()
    }

    fn collision_shape(&self) -> ShapeHandle<f64>; 

    fn collision_groups(&self) -> CollisionGroups;
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

