use crate::components::{Transform, Velocity};
use specs::{Join, ReadStorage, System, WriteStorage};
use nalgebra::Translation;

pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        // Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, velocities): Self::SystemData) {
        // TODO: Need some way to communicate time delta

        // Move all of the moving objects
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            // TODO: Multiply velocity components by time delta 
            transform.0.append_translation_mut(
                &Translation::from(velocity.vec));
        }
    }
}
