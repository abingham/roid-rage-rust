pub mod field;
pub mod objects;
pub mod object_set;
pub mod traits;

use nalgebra::{Isometry2, Vector2, zero};
use ncollide2d::pipeline::{CollisionGroups, CollisionObjectSlabHandle, ContactEvent, GeometricQueryType};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionWorld;
use std::collections::HashMap;

use field::Field;
use object_set::ObjectSet;
use traits::*;
use objects::roid::Roid;
use crate::velocity::{make_velocity_vector, random_bearing, Velocity};

pub struct Model {
    collision_world: CollisionWorld<f64, ()>,
    field: Field,
    roids: HashMap<CollisionObjectSlabHandle, Roid>
}

impl Model {
    pub fn new(
        field: Field,
        objects: ObjectSet,
    ) -> Model
    {
        let mut model = Model {
            collision_world: CollisionWorld::new(0.02f64),
            field: field,
            roids: HashMap::new(),
        };
        model.insert(objects);
        model
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn roids(&self) -> impl Iterator<Item=&Roid> {
        self.roids.values()
    }

    fn _project_group<I: Positioned + Identifiable + Exploding + Collidable>(
        &mut self, 
        objects: &mut HashMap::<CollisionObjectSlabHandle, I>, 
        collisions: &[CollisionObjectSlabHandle],
        time_delta: f64)
    {
        // Collect the objects that have exploded, removing them from the objects.
        let exploded: Vec<I> = collisions.iter()
            .filter_map(|handle| objects.remove(handle))
            .collect();

        // Move everything    
        for (_handle, obj) in &mut objects.iter_mut() {
            obj.project(&self.field, time_delta);
       }

        // Trim out the objects that have moved outside the field or otherwise died
        let mut removals: Vec<CollisionObjectSlabHandle> = objects.iter()
            .filter(|(_h, o)| !self.field.contains(&o.position()))
            .filter(|(_h, o)| !o.alive())
            .map(|(h, _o)| *h)
            .collect();
        removals.dedup();
        removals.sort();
        objects.retain(|handle, _obj| removals.binary_search(&handle).is_err());

        // Remove all dead objects from collision world.
        self.collision_world.remove(collisions);
        self.collision_world.remove(&removals);

        // Update the collision world for all remaining objects
        for (handle, obj) in objects {
            // Update collision object
            if let Some(collision_object) = self.collision_world.get_mut(*handle) {
                collision_object.set_position(
                    Isometry2::new(
                        Vector2::new(
                            obj.position().x, 
                            obj.position().y),
                    zero(),
                ));
            }
        }

        // Explode the collisions
        for ex in exploded {
            let new_objs = ex.explode();
            for roid in new_objs.roids {
                let (handle, _) = self.collision_world.add(
                    Isometry2::new(
                        Vector2::new(
                            roid.position().x, 
                            roid.position().y),
                        zero()),
                    roid.collision_shape(),
                    roid.collision_groups(),
                    GeometricQueryType::Contacts(0.0, 0.0),
                    ()
                );
                self.roids.insert(handle, roid);
            } 
        }
   } 

    // This is the core rule for updating the field. We bake this into the model because we treat it as the basic
    // "physics" of the game.
    pub fn project(&mut self, time_delta: f64) -> () {
        let collisions: Vec<CollisionObjectSlabHandle> = 
            self.collision_world.contact_events().iter()
            .filter_map(|event| {
                match event {
                    ContactEvent::Started(collider1, collider2) => {
                        Some(vec![*collider1, *collider2])
                    },
                    _ => None
                }
            }).flatten().collect();

        // Update roids
        // Collect the objects that have exploded, removing them from the objects.
        let exploded: Vec<Roid> = collisions.iter()
            .filter_map(|handle| self.roids.remove(handle))
            .collect();

        // Move everything    
        for (_handle, obj) in &mut self.roids.iter_mut() {
            obj.project(&self.field, time_delta);
        }

        // Trim out the objects that have moved outside the field or otherwise died
        let mut removals: Vec<CollisionObjectSlabHandle> = self.roids.iter()
            .filter(|(_h, o)| !self.field.contains(&o.position()))
            .filter(|(_h, o)| !o.alive())
            .map(|(h, _o)| *h)
            .collect();
        removals.dedup();
        removals.sort();
        self.roids.retain(|handle, _obj| removals.binary_search(&handle).is_err());

        // Remove all dead objects from collision world.
        self.collision_world.remove(&collisions);
        self.collision_world.remove(&removals);

        // Update the collision world for all remaining objects
        for (handle, obj) in &mut self.roids {
            // Update collision object
            if let Some(collision_object) = self.collision_world.get_mut(*handle) {
                collision_object.set_position(
                    Isometry2::new(
                        Vector2::new(
                            obj.position().x, 
                            obj.position().y),
                    zero(),
                ));
            }
        }

        // Explode the collisions
        for ex in exploded {
            self.insert(ex.explode());
        }

        self.collision_world.update();
    }

    fn insert(&mut self, objects: ObjectSet) -> () {
        for roid in objects.roids {
            let (handle, _) = self.collision_world.add(
                Isometry2::new(
                    Vector2::new(
                        roid.position().x, 
                        roid.position().y),
                    zero()),
                roid.collision_shape(),
                roid.collision_groups(),
                GeometricQueryType::Contacts(0.0, 0.0),
                ()
            );
            self.roids.insert(handle, roid);
        } 
    }
}

trait Exploding {
    fn explode(&self) -> ObjectSet;
}

impl Exploding for Roid {
    fn explode(&self) -> ObjectSet {
        let new_radius = self.radius() / 2.0;
        let num_sub_roids = if new_radius >= Roid::min_radius() { 2 } else { 0 };
        let new_roids = (0..num_sub_roids)
                .map(|_| {
                    let velocity = make_velocity_vector(self.velocity().speed() * 1.5, random_bearing());
                    Roid::new(self.position(), new_radius, velocity)
                });

        let mut objs = ObjectSet::new();
        objs.roids.extend(new_roids);
        objs
    }
}

trait Collidable {
    fn collision_shape(&self) -> ShapeHandle<f64>;
    fn collision_groups(&self) -> CollisionGroups;
}

impl Collidable for Roid {
    fn collision_shape(&self) -> ShapeHandle<f64> {
        ShapeHandle::new(Ball::new(self.radius()))
    }

    fn collision_groups(&self) -> CollisionGroups {
        let mut group = CollisionGroups::new();
        group.set_membership(&[ROID_GROUP]);
        group.set_whitelist(&[SHIP_GROUP, WEAPON_GROUP]);
        group
    }
}

const ROID_GROUP: usize = 1;
const SHIP_GROUP: usize = 2;
const WEAPON_GROUP: usize = 3;
const DEBRIS_GROUP: usize = 4;


