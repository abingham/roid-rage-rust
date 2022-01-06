#[derive(Clone)]
pub struct Rotation(pub f32);

impl specs::Component for Rotation {
    type Storage = specs::VecStorage<Self>;
}
