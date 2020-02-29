use super::collision_groups::{ROID_GROUP, WEAPON_GROUP};
use crate::components::{CollisionHandle, Velocity, Wrapping};
use amethyst::core::Transform;
use amethyst::prelude::{Builder, World, WorldExt};
use nalgebra::{zero, Isometry2, Vector2};
use ncollide2d::pipeline::{CollisionGroups, GeometricQueryType};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionWorld;

pub fn make_roid(world: &mut World, x: f32, y: f32) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.0);

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
        .with(Velocity::new(1.0, 0.0))
        .with(transform)
        .with(Wrapping)
        .with(CollisionHandle::new(collision_handle))
        .build();
}
