use crate::components::{LinearMotion, RotationalMotion, TimeDelta, Transform};
use nalgebra::{Translation, UnitComplex};
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct MoveObjectsSystem;

impl<'s> System<'s> for MoveObjectsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, LinearMotion>,
        ReadStorage<'s, RotationalMotion>,
        Read<'s, TimeDelta>,
    );

    fn run(&mut self, (mut transforms, linear_motions, rotational_motions, time_delta): Self::SystemData) {
        // Move all of the moving objects
        for (linear_motion, transform) in (&linear_motions, &mut transforms).join() {
            transform.0.append_translation_mut(&Translation::from(
                linear_motion.0 * time_delta.0.as_secs_f32(),
            ));
        }

        // Rotate all of the rotating objects
        for (rotational_motion, transform) in (&rotational_motions, &mut transforms).join() {
            transform.0.append_rotation_wrt_center_mut(
                &UnitComplex::new(rotational_motion.0)); 
        }
    }
}
