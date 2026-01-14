use glam::Vec2;
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

impl Velocity<f32> for Vec2
{
    fn dx(&self) -> f32 {
        self.x
    }

    fn dy(&self) -> f32 {
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

#[cfg(test)]
mod tests {
    use super::Velocity;

    #[test]
    fn tuple_speed_and_bearing() {
        let v = (3.0_f32, 4.0_f32);
        assert!((v.speed() - 5.0).abs() < 0.0001);
        assert!((v.bearing() - 0.9272952).abs() < 0.0001);
    }

    #[test]
    fn vec2_speed_and_bearing() {
        let v = glam::Vec2::new(-1.0, 0.0);
        assert!((v.speed() - 1.0).abs() < 0.0001);
        let bearing = v.bearing();
        assert!((bearing - std::f32::consts::PI).abs() < 0.0001);
    }
}
