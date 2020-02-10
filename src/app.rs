use crate::explosion::make_explosion;
use crate::field::Field;
use crate::game_object::GameObject;
use crate::objects::bullet::Bullet;
use crate::velocity::make_velocity_vector;
use nalgebra as na;
use ncollide2d::pipeline::CollisionObjectSlabHandle;
use ncollide2d::pipeline::{ContactEvent, GeometricQueryType};
use ncollide2d::world::CollisionWorld;
use opengl_graphics::GlGraphics;
use piston::input::*;
use std::collections::HashMap;

pub struct App {
    field: Field,
    game_objects: HashMap<CollisionObjectSlabHandle, Box<dyn GameObject>>,
    full_time: f64,
    collision_world: CollisionWorld<f64, Option<()>>,
}

const FIRING_FREQUENCY: f64 = 0.5;

impl App {
    pub fn new<I>(field: Field, game_objects: I) -> App
    where
        I: IntoIterator<Item = Box<dyn GameObject>>,
    {
        let mut app = App {
            field: field,
            game_objects: HashMap::<CollisionObjectSlabHandle, Box<dyn GameObject>>::new(),
            full_time: 0.0,
            collision_world: CollisionWorld::new(0.02f64),
        };

        for game_object in game_objects {
            app.insert(game_object);
        }

        app
    }

    fn insert(&mut self, game_object: Box<dyn GameObject>) -> () {
        let pos = na::Isometry2::new(
            na::Vector2::new(game_object.position()[0], game_object.position()[1]),
            na::zero(),
        );
        let (handle, _obj) = self.collision_world.add(
            pos,
            game_object.collision_shape(),
            game_object.collision_groups(),
            GeometricQueryType::Contacts(0.0, 0.0),
            None,
        );

        self.game_objects.insert(handle, game_object);
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for (_, roid) in &self.game_objects {
                roid.render(&WHITE, c, gl);
            }
        });
    }

    fn cleanup(&mut self) -> () {
        // Anything that should be removed goes on this vector.
        let mut removals: Vec<CollisionObjectSlabHandle> = vec![];
        let mut additions: Vec<Box<dyn GameObject>> = vec![];

        // Look for things that are off the field
        for (handle, game_object) in &self.game_objects {
            if !self.field.contains(game_object.position()) {
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
                if let Some(obj) = self.game_objects.get(&collider1) {
                    additions.extend(make_explosion(obj.position()));
                }
            }
        }

        // Remove those things
        removals.sort();
        removals.dedup();
        self.collision_world.remove(&removals);
        for removal in removals {
            self.game_objects.remove(&removal);
        }

        for addition in additions {
            self.insert(addition);
        }
    }

    pub fn update(&mut self, time_delta: f64) {
        // First cleanup up anything that should be removed
        self.cleanup();

        // Then update the collision world
        for (handle, roid) in &mut self.game_objects {
            roid.update(&self.field, time_delta);
            if let Some(object) = self.collision_world.get_mut(*handle) {
                let pos = na::Isometry2::new(
                    na::Vector2::new(roid.position()[0], roid.position()[1]),
                    na::zero(),
                );
                object.set_position(pos);
            }
        }
        self.collision_world.update();

        self.fire(time_delta);
    }

    fn fire(&mut self, dt: f64) -> () {
        let firing_position = na::Point2::new(
            (self.field.width() / 2) as f64,
            (self.field.height() / 2) as f64,
        );

        self.full_time += dt;
        if self.full_time > FIRING_FREQUENCY {
            self.full_time = 0.0;
            let bullet = Bullet::new(firing_position, make_velocity_vector(60.0, 0.0));
            self.insert(Box::new(bullet));

            // TODO: Use targeting, of course.
        }
    }
}
