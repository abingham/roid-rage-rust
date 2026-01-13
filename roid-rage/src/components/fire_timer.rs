#[derive(Clone, Copy, Debug)]
pub struct FireTimer(pub f32);

impl specs::Component for FireTimer {
    type Storage = specs::VecStorage<Self>;
}
