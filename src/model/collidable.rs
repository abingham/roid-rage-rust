use ncollide2d::pipeline::CollisionGroups;
use ncollide2d::shape::{Ball, ShapeHandle};
use super::objects::bullet::Bullet;
use super::objects::fragment::Fragment;
use super::objects::roid::Roid;

/// An object that can collide with other objects
pub trait Collidable {
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

impl Collidable for Fragment {
    fn collision_shape(&self) -> ShapeHandle<f64> {
        ShapeHandle::new(Ball::new(Fragment::radius()))
    }

    fn collision_groups(&self) -> CollisionGroups {
        let mut group = CollisionGroups::new();
        group.set_membership(&[DEBRIS_GROUP]);
        group.set_whitelist(&[]);
        group
    }
}

impl Collidable for Bullet {
    fn collision_shape(&self) -> ShapeHandle<f64> {
        ShapeHandle::new(Ball::new(Bullet::radius()))
    }

    fn collision_groups(&self) -> CollisionGroups {
        let mut group = CollisionGroups::new();
        group.set_membership(&[WEAPON_GROUP]);
        group.set_whitelist(&[ROID_GROUP]);
        group
    }
}

const ROID_GROUP: usize = 1;
const SHIP_GROUP: usize = 2;
const WEAPON_GROUP: usize = 3;
const DEBRIS_GROUP: usize = 4;

