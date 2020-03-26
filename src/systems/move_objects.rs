use crate::components::{AngularVelocity, LinearVelocity, TimeDelta, Transform};
use nalgebra::{Translation, UnitComplex};
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct MoveObjectsSystem;

impl<'s> System<'s> for MoveObjectsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, LinearVelocity>,
        ReadStorage<'s, AngularVelocity>,
        Read<'s, TimeDelta>,
    );

    fn run(
        &mut self,
        (mut transforms, linear_motions, angular_velocities, time_delta): Self::SystemData,
    ) {
        // Move all of the moving objects
        for (linear_motion, transform) in (&linear_motions, &mut transforms).join() {
            transform.0.append_translation_mut(&Translation::from(
                linear_motion.0 * time_delta.0.as_secs_f32(),
            ));
        }

        // Rotate all of the rotating objects
        for (angular_velocity, transform) in (&angular_velocities, &mut transforms).join() {
            transform
                .0
                .append_rotation_wrt_center_mut(&UnitComplex::new(angular_velocity.0));
        }
    }
}
