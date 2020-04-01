use nalgebra::Vector2;
use specs::Component;
use specs::VecStorage;

#[derive(Clone)]
pub struct Position(pub Vector2<f32>);

impl Component for Position {
    type Storage = VecStorage<Self>;
}
