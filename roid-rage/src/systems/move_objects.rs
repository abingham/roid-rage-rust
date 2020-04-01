use crate::components::{AngularVelocity, LinearVelocity, Position, Rotation, TimeDelta};
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct MoveObjectsSystem;

impl<'s> System<'s> for MoveObjectsSystem {
    type SystemData = (
        WriteStorage<'s, Position>,
        WriteStorage<'s, Rotation>,
        ReadStorage<'s, LinearVelocity>,
        ReadStorage<'s, AngularVelocity>,
        Read<'s, TimeDelta>,
    );

    fn run(
        &mut self,
        (mut positions, mut rotations, linear_velocities, angular_velocities, time_delta): Self::SystemData,
    ) {
        // Move all of the moving objects
        for (linear_velocity, position) in (&linear_velocities, &mut positions).join() {
            position.0 += linear_velocity.0 * time_delta.0.as_secs_f32();
        }

        // Rotate all of the rotating objects
        for (angular_velocity, rotation) in (&angular_velocities, &mut rotations).join() {
            rotation.0 += angular_velocity.0 * time_delta.0.as_secs_f32();
        }
    }
}
