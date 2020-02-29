use crate::components::{Velocity, Wrapping};
use crate::field::Field;
use amethyst::core::transform::Transform;
use amethyst::ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Wrapping>,
        ReadExpect<'s, Field>,
    );

    fn run(&mut self, (mut transforms, velocities, wrapping, field): Self::SystemData) {
        // TODO: Distance should be based on time-delta.

        // Processing entities that wrap
        for (velocity, transform, _) in (&velocities, &mut transforms, &wrapping).join() {
            let (new_x, new_y) = field.wrap(
                transform.translation().x + velocity.vec.x,
                transform.translation().y + velocity.vec.y,
            );

            transform.set_translation_y(new_y);
            transform.set_translation_x(new_x);
        }

        // Processing entities that don't wrap
        for (velocity, transform, _) in (&velocities, &mut transforms, !&wrapping).join() {
            transform.set_translation_x(transform.translation().x + velocity.vec.x);

            transform.set_translation_y(transform.translation().y + velocity.vec.y);
        }

        // TODO: Create new system to remove things that are off the field
    }
}
