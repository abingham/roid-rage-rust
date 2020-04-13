use crate::core::bearing::Bearing;
use specs::Component;
use specs::VecStorage;

#[derive(Clone)]
pub struct Rotation(pub Bearing);

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}
