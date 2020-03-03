use crate::components::Collision;
use crate::components::CollisionHandle;
use crate::components::Transform;
use nalgebra::{zero, Isometry2, Vector2};
use ncollide2d::pipeline::{CollisionObjectSlabHandle, ContactEvent};
use ncollide2d::world::CollisionWorld;
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};
use std::collections::HashSet;

pub struct CollisionDetectionSystem;

impl<'s> System<'s> for CollisionDetectionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, CollisionHandle>,
        WriteStorage<'s, Collision>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, specs::world::Index>>,
    );

    fn run(
        &mut self,
        (transforms, collision_handles, mut collision_markers, entities, mut collision_world): Self::SystemData,
    ) {
        for (transform, handle) in (&transforms, &collision_handles).join() {
            if let Some(collision_object) = collision_world.get_mut(handle.0) {
                collision_object.set_position(Isometry2::new(
                    Vector2::new(
                        transform.0.translation.x,
                        transform.0.translation.y,
                    ),
                    zero(),
                ));
            }
        }

        collision_world.update();

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


        // Record collisions
        for (handle, entity) in (&collision_handles, &entities).join() {
            if collisions.contains(&handle.0) {
                match collision_markers.insert(entity, Collision {}) {
                    Err(e) => println!("Error creating collision record: {}", e),
                    _ => {}
                }
            }
        }
    }
}
