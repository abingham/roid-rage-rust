use crate::components::{TimeDelta, Transform, Velocity};
use specs::{Join, ReadStorage, System, WriteStorage, Read};
use nalgebra::Translation;

pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, TimeDelta>,
    );

    fn run(&mut self, (mut transforms, velocities, time_delta): Self::SystemData) {
        // Move all of the moving objects
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            transform.0.append_translation_mut(
                &Translation::from(velocity.vec * time_delta.0.as_secs_f32()));
        }
    }
}
