use graphics;
use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use uuid;
use crate::object_set::ObjectSet;

use crate::field::Field;

pub trait Moving {
    fn position(&self) -> &Point2<f64>;
    fn velocity(&self) -> &Vector2<f64>;
    fn speed(&self) -> f64 {
        (self.velocity()[0].powf(2.0) + self.velocity()[1].powf(2.0)).sqrt()
    }
}

pub trait Updateable {
    fn update(&mut self, field: &Field, time_delta: f64);
}

pub trait Renderable {
    fn render(&self, 
              color: &[f32; 4], 
              c: graphics::Context, 
              gl: &mut GlGraphics);
}

pub trait Splode {
    fn splode(&self) -> ObjectSet;
}
