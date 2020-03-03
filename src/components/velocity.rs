use nalgebra::Vector2;
use specs::Component;
use specs::DenseVecStorage;

pub struct Velocity(pub Vector2<f32>);

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Velocity(Vector2::<f32>::new(dx, dy))
    }

    pub fn from_speed_and_bearing(speed: f32, bearing: f32) -> Self {
        Velocity(Vector2::<f32>::new(bearing.cos(), bearing.sin()) * speed)
    }

    pub fn speed(&self) -> f32 {
        (self.0.x.powf(2.0) + self.0.y.powf(2.0)).sqrt()
    }

    pub fn bearing(&self) -> f32 {
        self.0.y.atan2(self.0.x)
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

