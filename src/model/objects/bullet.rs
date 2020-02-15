use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, ShapeHandle};
use opengl_graphics::GlGraphics;
use rand::prelude::*;

use crate::model::field::Field;
use crate::model::object_set::ObjectSet;
use crate::model::objects::fragment::Fragment;
use crate::velocity::{make_velocity_vector, random_bearing};
use super::super::traits::{Identifiable, Positioned};

pub struct Bullet {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: uuid::Uuid,
}

impl Bullet {
    pub fn new(position: Point2<f64>, bearing: f64) -> Bullet {
        Bullet {
            position: position,
            velocity: make_velocity_vector(Bullet::speed(), bearing),
            id: uuid::Uuid::new_v4(),
        }
    }

    pub fn radius() -> f64 {
        2.0
    }

    pub fn speed() -> f64 {
        600.0
    }
}

impl Positioned for Bullet {
    fn position(&self) -> Point2<f64> {
        self.position
    }

    fn project(&mut self, _field: &Field, time_delta: f64) -> () {
        let new_position = self.position + self.velocity * time_delta;
        self.position = new_position;
    }
}

impl Identifiable for Bullet {
    fn id(&self) -> uuid::Uuid { self.id }
}
    // pub fn explode(&self) -> ObjectSet {
    //     let mut rng = thread_rng();
    //     let mut objects = ObjectSet::new();

    //     let fragments = (0..rng.gen_range(1, 10))
    //         .map(|_| {
    //             let speed = rng.gen::<f64>() * 400.0 + 200.0; 
    //             let max_age = rng.gen::<f64>() * 1.0;
    //             Fragment::new(
    //                 self.position,
    //                 make_velocity_vector(speed, random_bearing()),
    //                 0.0,
    //                 max_age,
    //             )
    //         });
    //     objects.fragments.extend(fragments);

    //     objects
    // }

    // // TODO: Re-add collidable trait
    // pub fn collision_shape(&self) -> ShapeHandle<f64> {
    //     ShapeHandle::new(Ball::new(Bullet::radius()))
    // }