use crate::components::{make_fragment, Bullet, Collision, CollisionHandle, Transform};
use crate::util::random_bearing;
use ncollide2d::pipeline::CollisionObjectSlabHandle;
use ncollide2d::world::CollisionWorld;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteExpect};

pub struct ExplodeBulletsSystem;

/// Explode roids that have collided with something.
impl<'s> System<'s> for ExplodeBulletsSystem {
    type SystemData = (
        ReadStorage<'s, CollisionHandle>,
        ReadStorage<'s, Collision>,
        ReadStorage<'s, Bullet>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, ()>>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (collision_handles, collisions, bullets, transforms, entities, mut collision_world, lazy): Self::SystemData,
    ) {
        let mut removals: Vec<CollisionObjectSlabHandle> = vec![];

        for (chandle, _collision, _bullet, transform, entity) in (
            &collision_handles,
            &collisions,
            &bullets,
            &transforms,
            &entities,
        )
            .join()
        {
            match entities.delete(entity) {
                Err(e) => println!("Error deleting bullet: {}", e),
                _ => {}
            }

            // TODO: Random number of fragments
            // TODO: Fragments should have randomized speeds
            for _ in 0..10 {
                let (vel, xform, bullet) = make_fragment(
                    transform.0.translation.x,
                    transform.0.translation.y,
                    random_bearing(),
                );

                let new_entity = entities.create();

                lazy.insert(new_entity, bullet);
                lazy.insert(new_entity, vel);
                lazy.insert(new_entity, xform);
            }
            removals.push(chandle.0);
        }

        collision_world.remove(&removals);
    }
}
