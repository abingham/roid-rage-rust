use crate::field::Field;
use amethyst::core::transform::Transform;
use amethyst::ecs::{Entities, Join, ReadExpect, ReadStorage, System};

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
            if !field.contains(transform.translation().x, transform.translation().y) {
                entities.delete(entity);
            }
        }
    }
}
