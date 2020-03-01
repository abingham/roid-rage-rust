use super::collision_groups::{ROID_GROUP, WEAPON_GROUP};
use crate::components::{CollisionHandle, Transform, Velocity, Wrapping};
use nalgebra::{zero, Isometry2, Vector2};
use ncollide2d::pipeline::{CollisionGroups, GeometricQueryType};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionWorld;
use specs::{Builder, World, WorldExt};

pub fn make_roid(world: &mut World, x: f32, y: f32) {
    let transform = Transform(Isometry2::new(Vector2::<f32>::new(x, y), 0.0f32));

    let mut collision_groups = CollisionGroups::new();
    collision_groups.set_membership(&[WEAPON_GROUP]);
    collision_groups.set_whitelist(&[ROID_GROUP]);

    let collision_isometry = Isometry2::new(Vector2::new(x as f64, y as f64), zero());

    let radius: f64 = 10.0;

    let collision_shape = ShapeHandle::new(Ball::new(radius));

    // Put entry in collision world
    let collision_world: &mut CollisionWorld<f64, ()> =
        world.get_mut::<CollisionWorld<f64, ()>>().unwrap();

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
        .with(Velocity::new(10.0, 10.0))
        .with(transform)
        .with(Wrapping)
        .with(CollisionHandle::new(collision_handle))
        .build();
}
