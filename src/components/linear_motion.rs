use nalgebra::Vector2;
use specs::Component;
use specs::DenseVecStorage;

pub struct LinearMotion(pub Vector2<f32>);

impl Component for LinearMotion {
    type Storage = DenseVecStorage<Self>;
}
