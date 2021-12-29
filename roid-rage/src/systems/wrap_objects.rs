use crate::components::{Position, Wrapping};
use crate::core::field::Field;
use glam::Vec2;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct WrapObjectsSystem;

/// Wrap entities that are supposed to wrap
impl<'s> System<'s> for WrapObjectsSystem {
    type SystemData = (
        WriteStorage<'s, Position>,
        ReadStorage<'s, Wrapping>,
        ReadExpect<'s, Field>,
    );

    fn run(&mut self, (mut positions, wrapping, field): Self::SystemData) {
        for (position, _wrapping) in (&mut positions, &wrapping).join() {
            let (x, y) = field.wrap(position.0.x, position.0.y);

            position.0 = Vec2::new(x, y);
        }
    }
}
