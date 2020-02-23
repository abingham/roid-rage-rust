use amethyst::ecs::prelude::Component;
use amethyst::ecs::storage::NullStorage;

#[derive(Default)]
pub struct Wrapping;

impl Component for Wrapping {
    type Storage = NullStorage<Self>;
}
