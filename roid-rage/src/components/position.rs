use glam::Vec2;
use specs::Component;
use specs::VecStorage;

#[derive(Clone)]
pub struct Position(pub Vec2);

impl Component for Position {
    type Storage = VecStorage<Self>;
}
