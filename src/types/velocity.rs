use nalgebra::Vector2;
use num::Float;

pub trait Velocity<T: Float> {
    fn dx(&self) -> T;

    fn dy(&self) -> T;

    fn speed(&self) -> T {
        (self.dx().powf(2.0) + self.dy().powf(2.0)).sqrt()
    }

    fn bearing(&self) -> T {
        self.dy().atan2(self.dx())
    }

}

impl<'a, T> Velocity<T> for Vector2<T>
    where T: Float + std::fmt::Debug
{
    fn dx(&self) -> T {
        self.x
    }

    fn dy(&self) -> T {
        self.y
    }
}

pub fn from_speed_and_bearing(speed: f32, bearing: f32) -> Vector2<f32> {
    Vector2::<f32>::new(bearing.cos(), bearing.sin()) * speed
}

