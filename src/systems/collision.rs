use crate::components::CollisionHandle;
use crate::components::Transform;
use nalgebra::{zero, Isometry2, Vector2};
use ncollide2d::pipeline::{CollisionObjectSlabHandle, ContactEvent};
use ncollide2d::world::CollisionWorld;
use specs::{Entities, Join, ReadStorage, System, WriteExpect};
use std::collections::HashSet;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, CollisionHandle>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, ()>>,
    );

    fn run(
        &mut self,
        (transforms, collision_handles, entities, mut collision_world): Self::SystemData,
    ) {
        for (transform, handle) in (&transforms, &collision_handles).join() {
            if let Some(collision_object) = collision_world.get_mut(handle.handle) {
                transform.0.translation.x;
                collision_object.set_position(Isometry2::new(
                    Vector2::new(
                        transform.0.translation.x,
                        transform.0.translation.y,
                    ),
                    zero(),
                ));
            }
        }

        // Find all collisions
        let collisions: HashSet<CollisionObjectSlabHandle> = collision_world
            .contact_events()
            .iter()
            .filter_map(|event| match event {
                ContactEvent::Started(collider1, collider2) => Some(vec![*collider1, *collider2]),
                _ => None,
            })
            .flatten()
            .collect();

        // Remove collided objects
        for (handle, entity) in (&collision_handles, &entities).join() {
            if collisions.contains(&handle.handle) {
                entities.delete(entity);
            }
        }

        // TODO: Added debris from collisions
    }
}
