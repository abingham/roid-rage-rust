use crate::components::Velocity;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        // Move all of the moving objects
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            transform.set_translation_x(
                transform.translation().x + velocity.vec.x * time.delta_seconds(),
            );
            transform.set_translation_y(
                transform.translation().y + velocity.vec.y * time.delta_seconds(),
            );
        }
    }
}
