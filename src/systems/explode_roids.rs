use crate::components::{make_roid, Collision, Roid, Transform, Velocity};
use crate::util::random_bearing;
use ncollide2d::world::CollisionWorld;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteExpect};

pub struct ExplodeRoidsSystem;

/// Explode roids that have collided with something.
impl<'s> System<'s> for ExplodeRoidsSystem {
    type SystemData = (
        ReadStorage<'s, Collision>,
        ReadStorage<'s, Roid>,
        ReadStorage<'s, Velocity>,
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
            velocities,
            transforms,
            entities,
            mut collision_world,
            lazy,
        ): Self::SystemData,
    ) {
        for (_, roid, vel, transform, entity) in
            (&collisions, &roids, &velocities, &transforms, &entities).join()
        {
            match entities.delete(entity) {
                Err(e) => println!("Error deleting roid: {}", e),
                _ => {}
            }

            if roid.radius >= Roid::min_radius() {
                for _ in 0..2 {
                    let new_entity = entities.create();
                    make_roid(
                        specs::world::LazyBuilder {
                            entity: new_entity,
                            lazy: &*lazy,
                        },
                        transform.0.translation.x,
                        transform.0.translation.y,
                        vel.speed() * 1.5,
                        random_bearing(),
                        roid.radius / 2.0,
                        &mut collision_world,
                    );
                }
            }
        }
    }
}
