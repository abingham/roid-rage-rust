use super::collision_groups::{ROID_GROUP, SHIP_GROUP, WEAPON_GROUP};
use crate::components::{
    AngularVelocity, CollisionHandle, LinearVelocity, Position, Rotation, Wrapping,
};
use crate::core::util::from_quantity_and_bearing;
use glam::Vec2;
use ncollide2d::na::{zero, Isometry2, Vector2};
use ncollide2d::pipeline::{CollisionGroups, GeometricQueryType};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionWorld;
use rand::Rng;
use specs::{Component, VecStorage};

pub struct Roid {
    pub radius: f32,
    pub points: Vec<f32>,
}

impl Roid {
    pub fn new(radius: f32, bumpiness: f32) -> Self {
        let mut rng = rand::rng();
        let num_points = rng.random::<u32>() % 5 + 5;
        let point_variance = radius * bumpiness;

        let points: Vec<f32> = (0..num_points)
            .map(|_| {
                vec![
                    radius + rng.random::<f32>() * point_variance,
                    radius - rng.random::<f32>() * point_variance,
                ]
            })
            .flatten()
            .collect();

        Roid {
            radius: radius,
            points: points,
        }
    }
}

impl Component for Roid {
    type Storage = VecStorage<Self>;
}

pub fn make_roid<B>(
    builder: B,
    x: f32,
    y: f32,
    speed: f32,
    course: f32,
    angular_velocity: f32,
    radius: f32,
    bumpiness: f32,
    collision_world: &mut CollisionWorld<f32, specs::world::Index>,
) where
    B: specs::world::Builder,
{
    let position = Position(Vec2::new(x, y));
    let rotation = Rotation(0.0);

    let mut collision_groups = CollisionGroups::new();
    collision_groups.set_membership(&[ROID_GROUP]);
    collision_groups.set_whitelist(&[SHIP_GROUP, WEAPON_GROUP]);

    let collision_isometry =
        Isometry2::new(Vector2::<f32>::new(position.0.x, position.0.y), zero());

    let collision_shape = ShapeHandle::new(Ball::new(radius));

    // Put entry in collision world
    let (collision_handle, obj) = collision_world.add(
        collision_isometry,
        collision_shape,
        collision_groups,
        GeometricQueryType::Contacts(0.0, 0.0),
        0,
    );

    // Create the entity
    let entity = builder
        .with(LinearVelocity(from_quantity_and_bearing(speed, course)))
        .with(AngularVelocity(angular_velocity))
        .with(position)
        .with(rotation)
        .with(Wrapping)
        .with(CollisionHandle(collision_handle))
        .with(Roid::new(radius, bumpiness))
        .build();

    // Annotate the collision object with the entity's ID
    *obj.data_mut() = entity.id();
}
