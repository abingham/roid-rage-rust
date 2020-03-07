use crate::components::{make_fragment, Bullet, Collision, Transform};
use crate::core::util::random_bearing;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System};
use rand::prelude::*;

pub struct ExplodeBulletsSystem;

/// Explode roids that have collided with something.
impl<'s> System<'s> for ExplodeBulletsSystem {
    type SystemData = (
        ReadStorage<'s, Collision>,
        ReadStorage<'s, Bullet>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (collisions, bullets, transforms, entities, lazy): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();

        for (_collision, _bullet, transform, entity) in (
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

            for _ in 0..(rng.next_u32() % 5 + 5) {
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
        }
    }
}
