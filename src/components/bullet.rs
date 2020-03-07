use super::collision_groups::{ROID_GROUP, WEAPON_GROUP};
use crate::components::{CollisionHandle, LinearMotion, Transform};
use crate::core::velocity::from_speed_and_bearing;
use nalgebra::{zero, Isometry2, Point2, Vector2};
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
        500.0
    }
}

impl Component for Bullet {
    // TODO: Is this the wrong storage type? Use something sparser?
    type Storage = VecStorage<Self>;
}

pub fn make_bullet<B>(
    builder: B,
    pos: Point2<f32>,
    bearing: f32,
    collision_world: &mut CollisionWorld<f32, specs::world::Index>,
) where
    B: specs::world::Builder,
{
    let isometry = Isometry2::new(Vector2::new(pos.x, pos.y), zero());
    let transform = Transform(isometry);

    let mut collision_groups = CollisionGroups::new();
    collision_groups.set_membership(&[WEAPON_GROUP]);
    collision_groups.set_whitelist(&[ROID_GROUP]);

    let collision_shape = ShapeHandle::new(Ball::new(Bullet::radius()));

    // Put entry in collision world
    let (collision_handle, obj) = collision_world.add(
        isometry,
        collision_shape,
        collision_groups,
        GeometricQueryType::Contacts(0.0, 0.0),
        0,
    );

    // Create the entity
    let entity = builder
        .with(Bullet::new())
        .with(LinearMotion(from_speed_and_bearing(
            Bullet::speed(),
            bearing,
        )))
        .with(transform)
        .with(CollisionHandle(collision_handle))
        .build();

    // Annotate the collision object with the entity's ID
    *obj.data_mut() = entity.id();
}
