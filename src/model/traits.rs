use nalgebra::Point2;
use uuid::Uuid;
use super::field::Field;

pub trait Positioned {
    fn position(&self) -> Point2<f64>;
    fn project(&mut self, field: &Field, time_delta: f64) -> ();
}

pub trait Identifiable {
    fn id(&self) -> Uuid;
    fn alive(&self) -> bool { true }
}