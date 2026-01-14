pub use self::age_fragments::AgeFragmentsSystem;
pub use self::cleanup_collisions::CleanupCollisionsSystem;
pub use self::detect_collisions::DetectCollisionsSystem;
pub use self::explode_bullets::ExplodeBulletsSystem;
pub use self::explode_roids::ExplodeRoidsSystem;
pub use self::move_objects::MoveObjectsSystem;
pub use self::register_pilots::PilotRegistrationSystem;
pub use self::query_pilot::QueryPilotSystem;
pub use self::remove_out_of_bounds::RemoveOutOfBoundsSystem;
pub use self::repopulate::RepopulateSystem;
pub use self::respawn_ship::RespawnShipSystem;
pub use self::wrap_objects::WrapObjectsSystem;

mod age_fragments;
mod cleanup_collisions;
mod detect_collisions;
mod explode_bullets;
mod explode_roids;
mod move_objects;
mod register_pilots;
mod query_pilot;
mod remove_out_of_bounds;
mod repopulate;
mod respawn_ship;
mod wrap_objects;

#[cfg(test)]
mod tests {
    use super::{MoveObjectsSystem, WrapObjectsSystem};
    use crate::components::{LinearVelocity, Position, TimeDelta, Wrapping};
    use crate::core::field::Field;
    use specs::{Builder, DispatcherBuilder, World, WorldExt};
    use std::time::Duration;

    #[test]
    fn move_then_wrap_dispatcher() {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<LinearVelocity>();
        world.register::<Wrapping>();
        world.insert(TimeDelta(Duration::from_secs_f32(1.0)));
        world.insert(Field::new(10.0_f32, 10.0_f32));

        world
            .create_entity()
            .with(Position(glam::Vec2::new(9.5, 5.0)))
            .with(LinearVelocity(glam::Vec2::new(2.0, 0.0)))
            .with(Wrapping)
            .build();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MoveObjectsSystem, "move", &[])
            .with(WrapObjectsSystem, "wrap", &["move"])
            .build();

        dispatcher.dispatch(&mut world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        let position = (&positions).join().next().unwrap();
        assert_eq!(position.0, glam::Vec2::new(0.0, 5.0));
    }
}
