use crate::components::{make_roid, AngularVelocity, Collision, LinearVelocity, Position, Roid};
use crate::core::util::random_bearing;
use sted::Velocity;
use crate::settings::Settings;
use ncollide2d::world::CollisionWorld;
use specs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect};

pub struct ExplodeRoidsSystem;

/// Explode roids that have collided with something.
impl<'s> System<'s> for ExplodeRoidsSystem {
    type SystemData = (
        ReadStorage<'s, Collision>,
        ReadStorage<'s, Roid>,
        ReadStorage<'s, LinearVelocity>,
        ReadStorage<'s, AngularVelocity>,
        ReadStorage<'s, Position>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, specs::world::Index>>,
        ReadExpect<'s, Settings>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            collisions,
            roids,
            linear_motions,
            angular_velocities,
            positions,
            entities,
            mut collision_world,
            settings,
            lazy,
        ): Self::SystemData,
    ) {
        for (_, roid, lm, av, position, entity) in (
            &collisions,
            &roids,
            &linear_motions,
            &angular_velocities,
            &positions,
            &entities,
        )
            .join()
        {
            match entities.delete(entity) {
                Err(e) => println!("Error deleting roid: {}", e),
                _ => {}
            }

            if roid.radius >= settings.minimum_roid_radius {
                for _ in 0..2 {
                    let new_entity = entities.create();
                    make_roid(
                        specs::world::LazyBuilder {
                            entity: new_entity,
                            lazy: &*lazy,
                        },
                        position.0.x,
                        position.0.y,
                        lm.0.speed() * 1.5,
                        random_bearing(),
                        av.0 * 2.0,
                        roid.radius / 2.0,
                        settings.roid_bumpiness,
                        &mut collision_world,
                    );
                }
            }
        }
    }
}
