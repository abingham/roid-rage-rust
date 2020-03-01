use crate::components::{Transform, CollisionHandle, Collision, Roid, Velocity, Wrapping};
use crate::components::collision_groups::{WEAPON_GROUP, ROID_GROUP};
use specs::{Entities, Join, LazyUpdate, Read, ReadExpect, WriteExpect, ReadStorage, System};
use rand::prelude::*;
use std::f32::consts::PI;
use ncollide2d::world::CollisionWorld;
use ncollide2d::pipeline::{CollisionGroups, GeometricQueryType};
use nalgebra::{Isometry2, zero};
use ncollide2d::shape::{Ball, ShapeHandle};

pub struct ExplodeRoidsSystem;

/// Explode roids that have collided with something.
impl<'s> System<'s> for ExplodeRoidsSystem {
    type SystemData = (
        ReadStorage<'s, Collision>,
        ReadStorage<'s, Roid>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, ()>>,
        Read<'s, LazyUpdate>
    );

    fn run(&mut self, (collisions, roids, velocities, transforms, entities, mut collision_world, lazy): Self::SystemData) {
        for (_, roid, vel, transform, entity) in (&collisions, &roids, &velocities, &transforms, &entities).join() {
            match entities.delete(entity) {
                Err(e) => println!("Error deleting roid: {}", e),
                _ => {}
            }

            if roid.radius >= Roid::min_radius() {
                let new_entity = entities.create();
                let fragment_radius = roid.radius / 2.0;

                // Create a smaller fragment
                lazy.insert(new_entity, Roid::new(fragment_radius));

                // It should be faster and move in a random direction
                lazy.insert(new_entity, Velocity::from_speed_and_bearing(
                    vel.speed() * 1.5,
                    random_bearing()));

                // Fragment is in the same position as the exploding roid
                lazy.insert(new_entity, transform.clone());

                // Fragments wrap
                lazy.insert(new_entity, Wrapping {});

                // And it needs a collision object
                let mut collision_groups = CollisionGroups::new();
                collision_groups.set_membership(&[WEAPON_GROUP]);
                collision_groups.set_whitelist(&[ROID_GROUP]);

                let collision_shape = ShapeHandle::new(Ball::new(fragment_radius));

                let (collision_handle, _) = collision_world.add(
                    transform.0,
                    collision_shape,
                    collision_groups,
                    GeometricQueryType::Contacts(0.0, 0.0),
                    (),
                );
                lazy.insert(new_entity, CollisionHandle::new(collision_handle));
            }
        }
    }
}

fn random_bearing() -> f32 {
    let mut rng = thread_rng();
    (rng.gen::<f32>() * 2.0 - 1.0) * PI
}

