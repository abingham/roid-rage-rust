use super::object_set::ObjectSet;
use super::objects::bullet::Bullet;
use super::objects::fragment::Fragment;
use super::objects::roid::Roid;
use super::traits::Positioned;
use crate::velocity::{make_velocity_vector, random_bearing, Velocity};
use rand::prelude::*;

pub trait Explodable {
    fn explode(&self) -> ObjectSet {
        ObjectSet::new()
    }
}

impl Explodable for Roid {
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

impl Explodable for Fragment {}

impl Explodable for Bullet {
    fn explode(&self) -> ObjectSet {
        let mut rng = thread_rng();
        let mut objects = ObjectSet::new();

        let fragments = (0..rng.gen_range(1, 10))
            .map(|_| {
                let speed = rng.gen::<f64>() * 400.0 + 200.0; 
                let max_age = rng.gen::<f64>() * 1.0;
                Fragment::new(
                    self.position(),
                    make_velocity_vector(speed, random_bearing()),
                    0.0,
                    max_age,
                )
            });
        objects.fragments.extend(fragments);

        objects
    }
}