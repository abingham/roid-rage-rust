use specs::Component;
use specs::NullStorage;

/// Created for entities that have collided with something
#[derive(Default)]
pub struct Collision;

impl Component for Collision {
    type Storage = NullStorage<Self>;
}
