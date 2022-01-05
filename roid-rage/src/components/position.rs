#[derive(Clone)]
pub struct Position(pub glam::Vec2);

impl specs::Component for Position {
    type Storage = specs::VecStorage<Self>;
}
