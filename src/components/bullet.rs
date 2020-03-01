use super::collision_groups::{ROID_GROUP, WEAPON_GROUP};
use crate::components::{CollisionHandle, Transform, Velocity, Wrapping};
use nalgebra::{zero, Isometry2, Vector2};
use ncollide2d::pipeline::{CollisionGroups, GeometricQueryType};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionWorld;
use specs::{Component, VecStorage};

pub struct Bullet {}

impl Bullet {
    pub fn new() -> Self {
        Bullet {}
    }

    pub fn radius() -> f32 {
        1.0
    }
    pub fn speed() -> f32 {
        20.0
    }
}

impl Component for Bullet {
    // TODO: Is this the wrong storage type? Use something sparser?
    type Storage = VecStorage<Self>;
}

pub fn make_bullet(
    x: f32,
    y: f32,
    bearing: f32,
    collision_world: &mut CollisionWorld<f32, ()>,
) -> (Velocity, Transform, CollisionHandle, Bullet) {
    let transform = Transform(Isometry2::new(Vector2::<f32>::new(x, y), 0.0f32));

    let mut collision_groups = CollisionGroups::new();
    collision_groups.set_membership(&[WEAPON_GROUP]);
    collision_groups.set_whitelist(&[ROID_GROUP]);

    let collision_isometry = Isometry2::new(Vector2::new(x, y), zero());

    let collision_shape = ShapeHandle::new(Ball::new(Bullet::radius()));

    // Put entry in collision world
    let (collision_handle, _) = collision_world.add(
        collision_isometry,
        collision_shape,
        collision_groups,
        GeometricQueryType::Contacts(0.0, 0.0),
        (),
    );

    (
        Velocity::from_speed_and_bearing(Bullet::speed(), bearing),
        transform,
        CollisionHandle::new(collision_handle),
        Bullet::new(),
    )
}
