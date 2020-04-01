use super::collision_groups::{ROID_GROUP, WEAPON_GROUP};
use crate::components::{CollisionHandle, LinearVelocity, Position};
use crate::core::velocity::from_speed_and_bearing;
use nalgebra::{Isometry2, Point2, Vector2};
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
}

impl Component for Bullet {
    // TODO: Is this the wrong storage type? Use something sparser?
    type Storage = VecStorage<Self>;
}

pub fn make_bullet<B>(
    builder: B,
    pos: Point2<f32>,
    speed: f32,
    bearing: f32,
    collision_world: &mut CollisionWorld<f32, specs::world::Index>,
) where
    B: specs::world::Builder,
{
    let position = Position(pos);

    let mut collision_groups = CollisionGroups::new();
    collision_groups.set_membership(&[WEAPON_GROUP]);
    collision_groups.set_whitelist(&[ROID_GROUP]);

    let collision_shape = ShapeHandle::new(Ball::new(Bullet::radius()));

    // Put entry in collision world
    let (collision_handle, obj) = collision_world.add(
        Isometry2::new(Vector2::<f32>::new(pos.x, pos.y), 0.0f32),
        collision_shape,
        collision_groups,
        GeometricQueryType::Contacts(0.0, 0.0),
        0,
    );

    // Create the entity
    let entity = builder
        .with(Bullet::new())
        .with(LinearVelocity(from_speed_and_bearing(speed, bearing)))
        .with(position)
        .with(CollisionHandle(collision_handle))
        .build();

    // Annotate the collision object with the entity's ID
    *obj.data_mut() = entity.id();
}
