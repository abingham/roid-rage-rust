use nalgebra::{Isometry2, Vector2, zero};
use ncollide2d::pipeline::{CollisionGroups, CollisionObjectSlabHandle, ContactEvent, GeometricQueryType};
use ncollide2d::world::CollisionWorld;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::collections::HashMap;

use super::game_object::{GameObject, Kind};
use super::field::Field;

pub struct Model {
    field: Field,
    game_objects: HashMap<CollisionObjectSlabHandle, Box<dyn GameObject>>,
    collision_world: CollisionWorld<f64, Option<()>>,
}

impl Model {
    pub fn new(field: Field) -> Model
    {
        Model {
            field: field,
            game_objects: HashMap::<CollisionObjectSlabHandle, Box<dyn GameObject>>::new(),
            collision_world: CollisionWorld::new(0.02f64),
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn insert(&mut self, game_object: Box<dyn GameObject>) -> () {
        let pos = Isometry2::new(
            Vector2::new(game_object.position()[0], game_object.position()[1]),
            zero(),
        );
        let (handle, _obj) = self.collision_world.add(
            pos,
            game_object.collision_shape(),
            Model::collision_groups(game_object.kind()),
            GeometricQueryType::Contacts(0.0, 0.0),
            None,
        );

        self.game_objects.insert(handle, game_object);
    }

    // This is the core rule for updating the field. We bake this into the model because we treat it as the basic
    // "physics" of the game.
    pub fn project(&mut self, time_delta: f64) -> () {

        // Move all of the game objects
        for game_object in self.game_objects.values_mut() {
            game_object.project(&self.field, time_delta);
        }

        self.cleanup();

        // Adjust collision objects for the game objects
        for (handle, game_object) in self.game_objects.iter_mut() {
            if let Some(object) = self.collision_world.get_mut(*handle) {
                let pos = Isometry2::new(
                    Vector2::new(game_object.position().x, game_object.position().y),
                    zero(),
                );
                object.set_position(pos);
            }
        }

        self.collision_world.update();
    }

    fn cleanup(&mut self) -> () {
        // Anything that should be removed goes on this vector.
        let mut removals: Vec<CollisionObjectSlabHandle> = vec![];
        let mut additions: Vec<Box<dyn GameObject>> = vec![];

        // Look for things that are off the field
        for (handle, game_object) in &self.game_objects {
            if !self.field.contains(game_object.position()) {
                removals.push(*handle);
            } else if !game_object.alive() {
                removals.push(*handle);
            }
        }

        // Then look for collisions
        for event in self.collision_world.contact_events() {
            if let &ContactEvent::Started(collider1, collider2) = event {
                for handle in vec![collider1, collider2] {
                    removals.push(handle);
                    if let Some(obj) = self.game_objects.get(&handle) {
                        additions.extend(obj.explode());
                    }
                }
            }
        }

        // Remove the things that should be removed
        removals.sort();
        removals.dedup();
        self.collision_world.remove(&removals);
        for removal in removals {
            self.game_objects.remove(&removal);
        }

        // Add the things that should be added
        for addition in additions {
            self.insert(addition);
        }
    }

    /// This determines an object's collision groups based on its "kind".
    fn collision_groups(kind: Kind) -> CollisionGroups {
        let mut group = CollisionGroups::new();

        match kind {
            Kind::Roid => {
                group.set_membership(&[ROID_GROUP]);
                group.set_whitelist(&[SHIP_GROUP, WEAPON_GROUP]);
            }
            Kind::Weapon => {
                group.set_membership(&[WEAPON_GROUP]);
                group.set_whitelist(&[ROID_GROUP]);
            }
            Kind::Ship => {
                group.set_membership(&[SHIP_GROUP]);
                group.set_whitelist(&[ROID_GROUP]);
            }
            Kind::Debris => {
                group.set_membership(&[DEBRIS_GROUP]);
            }
        }

        group
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for (_, roid) in &self.game_objects {
                roid.render(c, gl);
            }
        });
    }
}

// Collision groups
const ROID_GROUP: usize = 0;
const SHIP_GROUP: usize = 1;
const WEAPON_GROUP: usize = 2;
const DEBRIS_GROUP: usize = 3;

