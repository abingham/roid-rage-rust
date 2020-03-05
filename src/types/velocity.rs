use nalgebra::Vector2;

pub trait Velocity {
    fn dx(&self) -> f32;

    fn dy(&self) -> f32;

    fn speed(&self) -> f32 {
        (self.dx().powf(2.0) + self.dy().powf(2.0)).sqrt()
    }

    fn bearing(&self) -> f32 {
        self.dy().atan2(self.dx())
    }

}

impl Velocity for Vector2<f32> {
    fn dx(&self) -> f32 {
        self.x
    }

    fn dy(&self) -> f32 {
        self.y
    }
}

pub fn from_speed_and_bearing(speed: f32, bearing: f32) -> Vector2<f32> {
    Vector2::<f32>::new(bearing.cos(), bearing.sin()) * speed
}

