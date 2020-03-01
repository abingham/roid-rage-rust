use nalgebra::Vector2;
use specs::Component;
use specs::DenseVecStorage;

pub struct Velocity {
    pub vec: Vector2<f32>,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Velocity {
            vec: Vector2::<f32>::new(dx, dy),
        }
    }

    // pub fn from_speed_and_bearing(speed: f32, bearing: f32) -> Self {
    //     Velocity {
    //         vec: Vector2::<f32>::new(bearing.cos(), bearing.sin()) * speed,
    //     }
    // }

    // pub fn speed(&self) -> f32 {
    //     (self.vec.x.powf(2.0) + self.vec.y.powf(2.0)).sqrt()
    // }

    // pub fn bearing(&self) -> f32 {
    //     self.vec.y.atan2(self.vec.x)
    // }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
