use specs::Component;
use specs::DenseVecStorage;

type Radians = f32;

#[derive(Clone)]
pub struct Rotation(pub Radians);

impl Component for Rotation {
    type Storage = DenseVecStorage<Self>;
}
