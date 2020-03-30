use specs::Component;
use specs::NullStorage;

#[derive(Default)]
pub struct Wrapping;

impl Component for Wrapping {
    type Storage = NullStorage<Self>;
}
