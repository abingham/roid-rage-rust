use crate::components::{
    make_fragment, make_roid, Bullet, Collision, CollisionHandle, Fragment, Roid, TimeDelta,
    Transform, Velocity,
};
use crate::util::random_bearing;
use ncollide2d::pipeline::CollisionObjectSlabHandle;
use ncollide2d::world::CollisionWorld;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteExpect, WriteStorage};

pub struct AgeFragmentsSystem;

/// Explode roids that have collided with something.
impl<'s> System<'s> for AgeFragmentsSystem {
    type SystemData = (
        WriteStorage<'s, Fragment>,
        Entities<'s>,
        Read<'s, TimeDelta>,
    );

    fn run(&mut self, (mut fragments, entities, time_delta): Self::SystemData) {
        for (fragment, entity) in (&mut fragments, &entities).join() {
            fragment.age += time_delta.0;
            if fragment.age > fragment.max_age {
                match entities.delete(entity) {
                    Err(e) => println!("Error deleting fragment: {}", e),
                    _ => {}
                }
            }
        }
    }
}
