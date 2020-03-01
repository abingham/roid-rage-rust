use specs::Component;
use specs::DenseVecStorage;
use nalgebra::Isometry2;

pub struct Transform(pub Isometry2<f32>);

impl Component for Transform {
    type Storage = DenseVecStorage<Self>;
}
