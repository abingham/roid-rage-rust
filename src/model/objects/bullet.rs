use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, ShapeHandle};
use opengl_graphics::GlGraphics;
use rand::prelude::*;

use crate::model::field::Field;
use crate::model::game_object::{GameObject, Kind};
use crate::model::objects::fragment::Fragment;
use crate::velocity::{make_velocity_vector, random_bearing};

pub struct Bullet {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: uuid::Uuid,
}

impl Bullet {
    pub fn new(position: Point2<f64>, velocity: Vector2<f64>) -> Bullet {
        Bullet {
            position: position,
            velocity: velocity,
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

impl GameObject for Bullet {
    fn id(&self) -> uuid::Uuid { self.id }

    fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn project(&mut self, _field: &Field, time_delta: f64) -> () {
        let new_position = self.position + self.velocity * time_delta;
        self.position = new_position;
   }

   fn render(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(
            -1.0 * Bullet::radius(),
            -1.0 * Bullet::radius(),
            2.0 * Bullet::radius(),
        );
        ellipse([1.0, 1.0, 1.0, 1.0], rect, transform, gl);
    }

    fn explode(&self) -> Vec<Box<dyn GameObject>> {
        let mut rng = thread_rng();

        (0..rng.gen_range(1, 10))
            .map(|_| {
                let speed = rng.gen::<f64>() * 400.0 + 200.0; 
                let max_age = rng.gen::<f64>() * 1.0;
                Box::new(Fragment::new(
                    self.position,
                    make_velocity_vector(speed, random_bearing()),
                    0.0,
                    max_age,
                )) as Box<dyn GameObject>
            })
            .collect()
    }

    // TODO: Re-add collidable trait
    fn collision_shape(&self) -> ShapeHandle<f64> {
        ShapeHandle::new(Ball::new(Bullet::radius()))
    }

    fn kind(&self) -> Kind {
        Kind::Weapon
    }
}