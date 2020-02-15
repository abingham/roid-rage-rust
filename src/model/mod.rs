pub mod field;
pub mod objects;
pub mod object_set;
pub mod traits;

use nalgebra::{Isometry2, Vector2, zero};
use ncollide2d::pipeline::{CollisionGroups, CollisionObjectSlabHandle, ContactEvent, GeometricQueryType};
use ncollide2d::world::CollisionWorld;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::collections::hash_map::{HashMap, HashSet};

use field::Field;
use object_set::ObjectSet;
use traits::*;
use uuid::Uuid;
use objects::roid::Roid;
use crate::velocity::{make_velocity_vector, random_bearing};

pub struct Model {
    collision_world: CollisionWorld<f64, ()>,
    field: Field,
    roids: HashMap<CollisionObjectSlabHandle, Roid>
}

impl Model {
    pub fn new(field: Field) -> Model
    {
        Model {
            collision_world: CollisionWorld::new(0.02f64),
            field: field,
            roids: HashMap::new(),
        }
    }

    fn project_group<I: Positioned + Identifiable>(
        &self, 
        objects: &mut HashMap::<CollisionObjectSlabHandle, I>, 
        collisions: &HashSet<CollisionObjectSlabHandle>,
        time_delta: f64) ->  Vec<I>
    {
        // Collect the objects that have exploded, removing them from the objects.
        let explosions: Vec<I> = collisions.iter()
            .filter_map(|handle| objects.remove(handle))
            .collect();

        // Move everything    
        for (handle, obj) in objects {
            obj.project(&self.field, time_delta);

            // Update collision object
            if let Some(collision_object) = self.collision_world.get_mut(*handle) {
                collision_object.set_position(
                    Isometry2::new(
                    Vector2::new(obj.position().x, obj.position().y),
                    zero(),
                ));
            }
        }

        // Trim out the objects that have moved outside the field or otherwise died
        objects.retain(|handle, obj| self.field.contains(&obj.position()) && obj.alive());

        explosions
   } 

    // This is the core rule for updating the field. We bake this into the model because we treat it as the basic
    // "physics" of the game.
    pub fn project(&mut self, time_delta: f64) -> () {
        let collisions: HashSet<CollisionObjectSlabHandle> = self.collision_world.contact_events().iter()
            .filter_map(|event| {
                match event {
                    ContactEvent::Started(collider1, collider2) => {
                        Some([*collider1, *collider2])
                    },
                    _ => None
                }
            })
            .flat_map(|t| t.iter())
            .map(|h| *h)
            .collect();


        // Move all of the game objects
        let explosions = self.project_group(&self.roids, &self.field, time_delta);
        // project(&mut self.objects.bullets, &self.field, time_delta);
        // project(&mut self.objects.fragments, &self.field, time_delta);

        // for event in self.collision_world.contact_events() {
        //     if let &ContactEvent::Started(collider1, collider2) = event {

        //         for handle in vec![collider1, collider2] {
        //             removals.push(handle);
        //             if let Some(obj) = self.game_objects.get(&handle) {
        //                 additions.extend(obj.explode());
        //             }
        //         }
        //     }
        // }


        // // Adjust collision objects for the game objects
        // for (handle, game_object) in self.game_objects.iter_mut() {
        //     if let Some(object) = self.collision_world.get_mut(*handle) {
        //         let pos = Isometry2::new(
        //             Vector2::new(game_object.position().x, game_object.position().y),
        //             zero(),
        //         );
        //         object.set_position(pos);
        //     }
        // }

        self.collision_world.update();
    }

    fn cleanup(&mut self) -> () {
        // // Anything that should be removed goes on this vector.
        // let mut removals: Vec<CollisionObjectSlabHandle> = vec![];
        // let mut additions: Vec<Box<dyn GameObject>> = vec![];

        // // Then look for collisions
        // //
        // // TODO: In some cases a bullet intersects more than one roid. We should only explode one roid in this case.
        // // Remove the things that should be removed
        // removals.sort();
        // removals.dedup();
        // self.collision_world.remove(&removals);
        // for removal in removals {
        //     self.game_objects.remove(&removal);
        // }

        // // Add the things that should be added
        // for addition in additions {
        //     self.insert(addition);
        // }
    }

    // This determines an object's collision groups based on its "kind".
    // fn collision_groups(kind: Kind) -> CollisionGroups {
    //     let mut group = CollisionGroups::new();

    //     match kind {
    //         Kind::Roid => {
    //             group.set_membership(&[ROID_GROUP]);
    //             group.set_whitelist(&[SHIP_GROUP, WEAPON_GROUP]);
    //         }
    //         Kind::Weapon => {
    //             group.set_membership(&[WEAPON_GROUP]);
    //             group.set_whitelist(&[ROID_GROUP]);
    //         }
    //         Kind::Ship => {
    //             group.set_membership(&[SHIP_GROUP]);
    //             group.set_whitelist(&[ROID_GROUP]);
    //         }
    //         Kind::Debris => {
    //             group.set_membership(&[DEBRIS_GROUP]);
    //             group.set_blacklist(&[ROID_GROUP, DEBRIS_GROUP, WEAPON_GROUP]);
    //         }
    //     }

    //     group
    // }
}

// Collision groups
const ROID_GROUP: usize = 0;
const SHIP_GROUP: usize = 1;
const WEAPON_GROUP: usize = 2;
const DEBRIS_GROUP: usize = 3;

// fn project<I: Positioned>(objects: &HashMap<Uuid, I>, field: &Field, time_delta: f64) -> ()
// {
//     for (_, obj) in objects {
//         obj.project(field, time_delta);
//     }
// }

// fn remove<I: Positioned + Identifiable>(objects: &HashMap::<Uuid, I>, field: &Field) -> Vec<Uuid>
// {
//     objects.values()
//         .filter(|o| !field.contains(&o.position()))
//         .filter(|o| !o.alive())
//         .map(|o| o.id())
//         .collect()
// }

trait Exploding {
    fn explode(&self, model: &mut Model);
}

impl Exploding for Roid {
    fn explode(&self, model: &mut Model) {
        let new_radius = self.radius() / 2.0;
        let num_sub_roids = if new_radius >= Roid::min_radius() { 2 } else { 0 };
        let roids = (0..num_sub_roids)
            .map(|_| {
                let velocity = make_velocity_vector(self.velocity().speed() * 1.5, random_bearing());
                Roid::new(self.position, new_radius, velocity)
            });
        
        let mut objects = ObjectSet::new();
        objects.roids.extend(roids);
        objects
    }
}
