use crate::components::{make_fragment, Bullet, Collision, Transform};
use crate::util::random_bearing;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System};

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
        }
    }
}
