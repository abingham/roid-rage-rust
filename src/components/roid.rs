use super::collision_groups::{SHIP_GROUP, ROID_GROUP, WEAPON_GROUP};
use crate::components::{CollisionHandle, Transform, Velocity, Wrapping};
use nalgebra::{zero, Isometry2, Vector2};
use ncollide2d::pipeline::{CollisionGroups, GeometricQueryType};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionWorld;
use specs::{Component, VecStorage};

pub struct Roid {
    pub radius: f32,
}

impl Roid {
    pub fn new(radius: f32) -> Self {
        Roid { radius: radius }
    }

    pub fn min_radius() -> f32 {
        5.0
    }
}

impl Component for Roid {
    // TODO: Is this the wrong storage type? Use something sparser?
    type Storage = VecStorage<Self>;
}

pub fn make_roid(
    x: f32,
    y: f32,
    speed: f32,
    bearing: f32,
    radius: f32,
    collision_world: &mut CollisionWorld<f32, ()>,
) -> (Velocity, Transform, Wrapping, CollisionHandle, Roid) {
    let transform = Transform(Isometry2::new(Vector2::<f32>::new(x, y), 0.0f32));

    let mut collision_groups = CollisionGroups::new();
    collision_groups.set_membership(&[ROID_GROUP]);
    collision_groups.set_whitelist(&[SHIP_GROUP, WEAPON_GROUP]);

    let collision_isometry = Isometry2::new(Vector2::new(x, y), zero());

    let collision_shape = ShapeHandle::new(Ball::new(radius));

    // Put entry in collision world
    let (collision_handle, _) = collision_world.add(
        collision_isometry,
        collision_shape,
        collision_groups,
        GeometricQueryType::Contacts(0.0, 0.0),
        (),
    );

    (
        Velocity::from_speed_and_bearing(speed, bearing),
        transform,
        Wrapping {},
        CollisionHandle(collision_handle),
        Roid::new(radius),
    )
}
