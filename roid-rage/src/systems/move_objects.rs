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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{AngularVelocity, LinearVelocity, Position, Rotation, TimeDelta};
    use specs::{Builder, World, WorldExt};
    use std::time::Duration;

    #[test]
    fn integrates_position_and_rotation() {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Rotation>();
        world.register::<LinearVelocity>();
        world.register::<AngularVelocity>();
        world.insert(TimeDelta(Duration::from_secs_f32(0.5)));

        world
            .create_entity()
            .with(Position(glam::Vec2::new(1.0, 2.0)))
            .with(Rotation(0.0))
            .with(LinearVelocity(glam::Vec2::new(4.0, -2.0)))
            .with(AngularVelocity(2.0))
            .build();

        let mut system = MoveObjectsSystem;
        system.run_now(&world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        let rotations = world.read_storage::<Rotation>();
        let (position, rotation) = (&positions, &rotations).join().next().unwrap();

        assert_eq!(position.0, glam::Vec2::new(3.0, 1.0));
        assert!((rotation.0 - 1.0).abs() < 0.0001);
    }
}
