use crate::components::Transform;
use crate::field::Field;
use specs::{Entities, Join, ReadExpect, ReadStorage, System};

pub struct OutOfBoundsSystem;

/// Delete entities that have left the field
impl<'s> System<'s> for OutOfBoundsSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        Entities<'s>,
        ReadExpect<'s, Field>,
    );

    fn run(&mut self, (transforms, entities, field): Self::SystemData) {
        for (transform, entity) in (&transforms, &entities).join() {
            if !field.contains(transform.0.translation.x, transform.0.translation.y) {
                match entities.delete(entity) {
                    Err(e) => println!("Error deleting entity: {}", e),
                    _ => {}
                }
            }
        }
    }
}
