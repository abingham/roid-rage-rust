use nalgebra::Isometry2;
use specs::Component;
use specs::DenseVecStorage;

#[derive(Clone)]
pub struct Transform(pub Isometry2<f32>);

impl Component for Transform {
    type Storage = DenseVecStorage<Self>;
}
