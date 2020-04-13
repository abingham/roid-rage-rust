use nalgebra::Vector2;
use num::{Float, FromPrimitive};

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
where
    T: Float + FromPrimitive + std::fmt::Debug + 'static,
{
    fn dx(&self) -> T {
        self.x
    }

    fn dy(&self) -> T {
        self.y
    }
}

impl<T> Velocity<T> for (T, T)
where
    T: Float + FromPrimitive + std::fmt::Debug + 'static,
{
    fn dx(&self) -> T {
        self.0
    }

    fn dy(&self) -> T {
        self.1
    }
}