use amethyst::core::transform::Transform;
use crate::components::Velocity;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};

pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>
    );

    fn run(&mut self, (mut transforms, velocities): Self::SystemData) {
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            // TODO: Need to distinguish between objects that wrap and those which exit the field. A marker component with null storage.
            // TODO: Handle X
            // TODO: Distance should be based on time-delta.

            let start_y = transform.translation().y;
            transform.set_translation_y(start_y + velocity.vec.y);

            let start_x = transform.translation().x;
            transform.set_translation_y(start_x + velocity.vec.x);
       }
    }
}
