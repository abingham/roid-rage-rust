use nalgebra::{Point2, Vector2};
use ncollide2d::pipeline::CollisionGroups;
use ncollide2d::shape::{Ball, ShapeHandle};
use opengl_graphics::GlGraphics;
use crate::explosion::make_explosion;

use crate::game_object::{GameObject, MASSIVE_GROUP, WEAPON_GROUP};
use crate::field::Field;

pub struct Bullet {
    // collision_shape: Ball<f64>,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: uuid::Uuid
}

impl Bullet {
    pub fn new(position: Point2<f64>, velocity: Vector2<f64>) -> Bullet {
        Bullet {
            position: position,
            velocity: velocity,
            // collision_shape: Ball::new(Bullet::radius()),
            id: uuid::Uuid::new_v4()
        }
    }

    pub fn radius() -> f64 {
        2.0
    }

    pub fn speed() -> f64 {
        600.0
    }
}

impl GameObject for Bullet {
    fn id(&self) -> uuid::Uuid { self.id }

    fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }

    fn update(&mut self, field: &Field, time_delta: f64) -> () {
        let new_position = self.position + self.velocity * time_delta;
        self.position = new_position;
   }

   fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(
            -1.0 * Bullet::radius(),
            -1.0 * Bullet::radius(),
            2.0 * Bullet::radius(),
        );
        ellipse(*color, rect, transform, gl);
    }

    fn explode(&self) -> Vec<Box<dyn GameObject>> {
        make_explosion(&self.position)
    }

    // TODO: Re-add collidable trait
    fn collision_shape(&self) -> ShapeHandle<f64> {
        ShapeHandle::new(Ball::new(Bullet::radius()))
    }

    fn collision_groups(&self) -> CollisionGroups {
        let mut group = CollisionGroups::new();
        group.set_membership(&[WEAPON_GROUP]);
        group.set_whitelist(&[MASSIVE_GROUP]);
        group
    }
}