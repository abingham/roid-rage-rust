use specs::Component;
use specs::VecStorage;
use std::f32::consts::PI;
use crate::core::bearing::Bearing;

#[derive(Clone)]
pub struct Rotation(pub Bearing);

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}
