use nalgebra::Vector2;
use num::{FromPrimitive, Float, NumCast};

pub trait Velocity<T: Float + FromPrimitive> {
    fn dx(&self) -> T;

    fn dy(&self) -> T;

    fn speed(&self) -> T {
        let two = FromPrimitive::from_u8(2).unwrap();
        (self.dx().powf(two) + self.dy().powf(two)).sqrt()
    }

    fn bearing(&self) -> T {
        self.dy().atan2(self.dx())
    }
}

impl<T> Velocity<T> for Vector2<T>
    where T: Float + FromPrimitive + std::fmt::Debug + 'static
{
    fn dx(&self) -> T {
        NumCast::from::<T>(self.x).unwrap()
    }

    fn dy(&self) -> T {
        self.y
    }
}

pub fn from_speed_and_bearing(speed: f32, bearing: f32) -> Vector2<f32> {
    Vector2::<f32>::new(bearing.cos(), bearing.sin()) * speed
}