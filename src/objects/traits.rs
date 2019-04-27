use graphics;
use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use uuid::Uuid;
use ncollide2d::shape::Shape;

pub trait Renderable {
    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics);
}

pub trait Positioned {
    fn update(&mut self, time_delta: f64) {
        self.set_position(self.position() + self.velocity() * time_delta);
    }

    fn position(&self) -> &Point2<f64>;
    fn set_position(&mut self, pos: Point2<f64>);
    fn velocity(&self) -> &Vector2<f64>;

}

pub trait Identifiable {
    fn id(&self) -> Uuid;
}

pub trait Collidable: Positioned + Identifiable {
    fn collision_shape(&self) -> &Shape<f64>;
}

pub trait Mortal {
    fn alive(&self) -> bool;
    fn kill(&mut self);
}