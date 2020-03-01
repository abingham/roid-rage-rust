use super::collision_groups::{ROID_GROUP, WEAPON_GROUP};
use crate::components::{CollisionHandle, Transform, Velocity, Wrapping};
use nalgebra::{zero, Isometry2, Vector2};
use ncollide2d::pipeline::{CollisionGroups, GeometricQueryType};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionWorld;
use specs::{Component, VecStorage, Builder, World, WorldExt};


pub struct Roid {
    pub radius: f32
}

impl Roid {
    pub fn new(radius: f32) -> Self {
        Roid {
            radius: radius
        }
    }

    pub fn min_radius() -> f32 { 5.0 }
}

impl Component for Roid {
    // TODO: Is this the wrong storage type? Use something sparser?
    type Storage = VecStorage<Self>;
}

// TODO: Refactor. This is largely duplicated in the roid explosion code.
pub fn make_roid(world: &mut World, x: f32, y: f32) {
    let transform = Transform(Isometry2::new(Vector2::<f32>::new(x, y), 0.0f32));

    let mut collision_groups = CollisionGroups::new();
    collision_groups.set_membership(&[WEAPON_GROUP]);
    collision_groups.set_whitelist(&[ROID_GROUP]);

    let collision_isometry = Isometry2::new(Vector2::new(x, y), zero());

    let radius: f32 = 10.0;

    let collision_shape = ShapeHandle::new(Ball::new(radius));

    // Put entry in collision world
    let collision_world: &mut CollisionWorld<f32, ()> =
        world.get_mut::<CollisionWorld<f32, ()>>().unwrap();

    let (collision_handle, _) = collision_world.add(
        collision_isometry,
        collision_shape,
        collision_groups,
        GeometricQueryType::Contacts(0.0, 0.0),
        (),
    );

    // Create a roid entity
    world
        .create_entity()
        .with(Velocity::new(2.0, 2.0))
        .with(transform)
        .with(Wrapping)
        .with(CollisionHandle::new(collision_handle))
        .with(Roid::new(10.0))
        .build();
}
