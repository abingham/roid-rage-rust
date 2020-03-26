use crate::components::{LinearMotion, TimeDelta, Transform};
use nalgebra::{Translation, UnitComplex};
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct MoveObjectsSystem;

impl<'s> System<'s> for MoveObjectsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, LinearMotion>,
        Read<'s, TimeDelta>,
    );

    fn run(&mut self, (mut transforms, linear_motions, time_delta): Self::SystemData) {
        // Move all of the moving objects
        for (linear_motion, transform) in (&linear_motions, &mut transforms).join() {
            // TODO: Each roid should have its own rotational velocity (i.e. in a component)
            let rot = UnitComplex::new(0.01);

            transform.0.append_rotation_wrt_center_mut(&rot);

            transform.0.append_translation_mut(&Translation::from(
                linear_motion.0 * time_delta.0.as_secs_f32(),
            ));
        }
    }
}
