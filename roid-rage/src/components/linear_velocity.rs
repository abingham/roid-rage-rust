use glam::Vec2;
use specs::Component;
use specs::VecStorage;

pub struct LinearVelocity(pub Vec2);

impl Component for LinearVelocity {
    type Storage = VecStorage<Self>;
}
