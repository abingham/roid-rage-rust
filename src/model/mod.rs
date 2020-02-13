pub mod field;
pub mod objects;
pub mod object_set;

use nalgebra::{Isometry2, Vector2, zero};
use ncollide2d::pipeline::{CollisionGroups, CollisionObjectSlabHandle, ContactEvent, GeometricQueryType};
use ncollide2d::world::CollisionWorld;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::collections::hash_map::HashMap;

use field::Field;
use object_set::ObjectSet;

pub struct Model {
    collision_world: CollisionWorld<f64, ()>,
    pub field: Field,
    pub objects: ObjectSet,
}

impl Model {
    pub fn new(field: Field) -> Model
    {
        Model {
            collision_world: CollisionWorld::new(0.02f64),
            field: field,
            objects: ObjectSet::new()
        }
    }

    // This is the core rule for updating the field. We bake this into the model because we treat it as the basic
    // "physics" of the game.
    pub fn project(&mut self, time_delta: f64) -> () {

        // Move all of the game objects
        // for game_object in self.game_objects.values_mut() {
        //     game_object.project(&self.field, time_delta);
        // }

        // self.cleanup();

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

        // self.collision_world.update();
    }

    fn cleanup(&mut self) -> () {
        // // Anything that should be removed goes on this vector.
        // let mut removals: Vec<CollisionObjectSlabHandle> = vec![];
        // let mut additions: Vec<Box<dyn GameObject>> = vec![];

        // // Look for things that are off the field
        // for (handle, game_object) in &self.game_objects {
        //     if !self.field.contains(game_object.position()) {
        //         removals.push(*handle);
        //     } else if !game_object.alive() {
        //         removals.push(*handle);
        //     }
        // }

        // // Then look for collisions
        // //
        // // TODO: In some cases a bullet intersects more than one roid. We should only explode one roid in this case.
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

