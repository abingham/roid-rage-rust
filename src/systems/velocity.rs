use crate::components::Velocity;
use amethyst::core::transform::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Velocity>);

    fn run(&mut self, (mut transforms, velocities): Self::SystemData) {
        // TODO: Distance should be based on time-delta.

        // Processing entities that don't wrap
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            transform.set_translation_x(transform.translation().x + velocity.vec.x);
            transform.set_translation_y(transform.translation().y + velocity.vec.y);
        }

        // TODO: Create new system to remove things that are off the field
    }
}
