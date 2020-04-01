use crate::components::Position;
use crate::core::field::Field;
use specs::{Entities, Join, ReadExpect, ReadStorage, System};

pub struct RemoveOutOfBoundsSystem;

/// Delete entities that have left the field
impl<'s> System<'s> for RemoveOutOfBoundsSystem {
    type SystemData = (
        ReadStorage<'s, Position>,
        Entities<'s>,
        ReadExpect<'s, Field>,
    );

    fn run(&mut self, (positions, entities, field): Self::SystemData) {
        for (position, entity) in (&positions, &entities).join() {
            if !field.contains(position.0.x, position.0.y) {
                match entities.delete(entity) {
                    Err(e) => println!("Error deleting entity: {}", e),
                    _ => {}
                }
            }
        }
    }
}
