use crate::components::{make_roid, AngularVelocity, Collision, LinearVelocity, Roid, Transform};
use crate::core::util::random_bearing;
use crate::core::velocity::Velocity;
use ncollide2d::world::CollisionWorld;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteExpect};

pub struct ExplodeRoidsSystem {
    min_roid_radius: f32,
    roid_bumpiness: f32,
}

impl ExplodeRoidsSystem {
    pub fn new(min_roid_radius: f32, roid_bumpiness: f32) -> ExplodeRoidsSystem {
        ExplodeRoidsSystem {
            min_roid_radius: min_roid_radius,
            roid_bumpiness: roid_bumpiness,
        }
    }
}

/// Explode roids that have collided with something.
impl<'s> System<'s> for ExplodeRoidsSystem {
    type SystemData = (
        ReadStorage<'s, Collision>,
        ReadStorage<'s, Roid>,
        ReadStorage<'s, LinearVelocity>,
        ReadStorage<'s, AngularVelocity>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, specs::world::Index>>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            collisions,
            roids,
            linear_motions,
            angular_velocities,
            transforms,
            entities,
            mut collision_world,
            lazy,
        ): Self::SystemData,
    ) {
        for (_, roid, lm, av, transform, entity) in (
            &collisions,
            &roids,
            &linear_motions,
            &angular_velocities,
            &transforms,
            &entities,
        )
            .join()
        {
            match entities.delete(entity) {
                Err(e) => println!("Error deleting roid: {}", e),
                _ => {}
            }

            if roid.radius >= self.min_roid_radius {
                for _ in 0..2 {
                    let new_entity = entities.create();
                    make_roid(
                        specs::world::LazyBuilder {
                            entity: new_entity,
                            lazy: &*lazy,
                        },
                        transform.0.translation.x,
                        transform.0.translation.y,
                        lm.0.speed() * 1.5,
                        random_bearing(),
                        av.0 * 2.0,
                        roid.radius / 2.0,
                        self.roid_bumpiness,
                        &mut collision_world,
                    );
                }
            }
        }
    }
}
