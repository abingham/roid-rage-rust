use crate::components::{make_fragment, Bullet, Collision, Position};
use crate::core::util::random_bearing;
use rand::Rng;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System};

pub struct ExplodeBulletsSystem;

/// Explode bullets that have collided with something.
impl<'s> System<'s> for ExplodeBulletsSystem {
    type SystemData = (
        ReadStorage<'s, Collision>,
        ReadStorage<'s, Bullet>,
        ReadStorage<'s, Position>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (collisions, bullets, positions, entities, lazy): Self::SystemData) {
        let mut rng = rand::rng();

        for (_collision, _bullet, position, entity) in
            (&collisions, &bullets, &positions, &entities).join()
        {
            match entities.delete(entity) {
                Err(e) => println!("Error deleting bullet: {}", e),
                _ => {}
            }

            for _ in 0..(rng.random::<u32>() % 5 + 5) {
                let new_entity = entities.create();

                make_fragment(
                    specs::world::LazyBuilder {
                        entity: new_entity,
                        lazy: &*lazy,
                    },
                    position.0.x,
                    position.0.y,
                    random_bearing(),
                );
            }
        }
    }
}
