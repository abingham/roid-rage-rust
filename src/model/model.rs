use nalgebra::{zero, Isometry2, Vector2};
use ncollide2d::pipeline::{CollisionObjectSlabHandle, ContactEvent, GeometricQueryType};
use ncollide2d::world::CollisionWorld;
use std::collections::HashMap;

use super::collidable::Collidable;
use super::field::Field;
use super::object_map::ObjectMap;
use super::object_set::ObjectSet;
use super::objects::bullet::Bullet;
use super::objects::fragment::Fragment;
use super::objects::roid::Roid;
use super::traits::*;

pub struct Model {
    collision_world: CollisionWorld<f64, ()>,
    field: Field,
    bullets: HashMap<CollisionObjectSlabHandle, Bullet>,
    roids: HashMap<CollisionObjectSlabHandle, Roid>,
    fragments: HashMap<CollisionObjectSlabHandle, Fragment>,
}

impl Model {
    pub fn new(field: Field, objects: ObjectSet) -> Model {
        let mut model = Model {
            collision_world: CollisionWorld::new(0.02f64),
            field: field,
            bullets: HashMap::new(),
            fragments: HashMap::new(),
            roids: HashMap::new(),
        };
        model.insert(objects);
        model
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn roids(&self) -> impl Iterator<Item = &Roid> {
        self.roids.values()
    }

    pub fn bullets(&self) -> impl Iterator<Item = &Bullet> {
        self.bullets.values()
    }

    pub fn fragments(&self) -> impl Iterator<Item = &Fragment> {
        self.fragments.values()
    }

    // This is the core rule for updating the field. We bake this into the model because we treat it as the basic
    // "physics" of the game.
    pub fn project(&mut self, time_delta: f64) -> () {
        // Find all of the handles for objects that are collided.
        let mut collisions: Vec<CollisionObjectSlabHandle> = self
            .collision_world
            .contact_events()
            .iter()
            .filter_map(|event| match event {
                ContactEvent::Started(collider1, collider2) => Some(vec![*collider1, *collider2]),
                _ => None,
            })
            .flatten()
            .collect();
        collisions.dedup();

        // Ask the objects groups to update their objects, report objects to remove, and any debris they've generated.
        let mut removals: Vec<CollisionObjectSlabHandle> = vec![];
        let mut debris: ObjectSet = ObjectSet::new();

        // TODO: It would be nice to be able to abstract over these collections, but I run into
        // borrow checker issues when I try.
        let (r, d) = self.roids.project(time_delta, &collisions, &self.field);
        removals.extend(r);
        debris.extend(d);

        let (r, d) = self.fragments.project(time_delta, &collisions, &self.field);
        removals.extend(r);
        debris.extend(d);

        let (r, d) = self.bullets.project(time_delta, &collisions, &self.field);
        removals.extend(r);
        debris.extend(d);

        // This is close!§
        // Calculate all of the collision objects to remove as well as the debris to add to the model.
        // let (mut removals, debris) = groups.iter_mut().fold(
        //     (vec![], ObjectSet::new()),
        //     |(mut removals, mut debris), g| {
        //         let (r, d) = g.project(time_delta, &collisions, &self.field);
        //         removals.extend(r);
        //         debris.extend(d);
        //         (removals, debris)
        //     });

        removals.dedup();
        self.collision_world.remove(&removals);

        let positions = self
            .roids
            .iter()
            .map(|(h, o)| (*h, o.position()))
            .chain(self.fragments.iter().map(|(h, o)| (*h, o.position())))
            .chain(self.bullets.iter().map(|(h, o)| (*h, o.position())));

        // Update position of all collision objects
        for (handle, pos) in positions {
            // Update collision object
            if let Some(collision_object) = self.collision_world.get_mut(handle) {
                collision_object.set_position(Isometry2::new(Vector2::new(pos.x, pos.y), zero()));
            }
        }

        self.insert(debris);

        self.collision_world.update();
    }

    pub fn insert(&mut self, objects: ObjectSet) -> () {
        for roid in objects.roids {
            let (handle, _) = self.collision_world.add(
                Isometry2::new(Vector2::new(roid.position().x, roid.position().y), zero()),
                roid.collision_shape(),
                roid.collision_groups(),
                GeometricQueryType::Contacts(0.0, 0.0),
                (),
            );
            self.roids.insert(handle, roid);
        }

        for fragment in objects.fragments {
            let (handle, _) = self.collision_world.add(
                Isometry2::new(
                    Vector2::new(fragment.position().x, fragment.position().y),
                    zero(),
                ),
                fragment.collision_shape(),
                fragment.collision_groups(),
                GeometricQueryType::Contacts(0.0, 0.0),
                (),
            );
            self.fragments.insert(handle, fragment);
        }
        for bullet in objects.bullets {
            let (handle, _) = self.collision_world.add(
                Isometry2::new(
                    Vector2::new(bullet.position().x, bullet.position().y),
                    zero(),
                ),
                bullet.collision_shape(),
                bullet.collision_groups(),
                GeometricQueryType::Contacts(0.0, 0.0),
                (),
            );
            self.bullets.insert(handle, bullet);
        }
    }
}
