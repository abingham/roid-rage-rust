#[derive(Clone)]
pub struct Rotation(pub glam::Vec2);

impl specs::Component for Rotation {
    type Storage = specs::VecStorage<Self>;
}
