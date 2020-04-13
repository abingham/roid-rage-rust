use sted::Bearing;
use specs::Component;
use specs::VecStorage;

#[derive(Clone)]
pub struct Rotation(pub Bearing<f32>);

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}
