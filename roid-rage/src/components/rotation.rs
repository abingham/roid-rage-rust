use specs::Component;
use specs::VecStorage;

type Radians = f32;

#[derive(Clone)]
pub struct Rotation(pub Radians);

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}
