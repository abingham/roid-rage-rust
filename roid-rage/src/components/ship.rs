use crate::components::{AngularVelocity, LinearVelocity, Transform, Wrapping};
use crate::core::velocity::from_speed_and_bearing;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::world::CollisionWorld;
use specs::{Component, VecStorage};

pub struct Ship {
    pub heading: f32,
    pub length: f32,
    pub width: f32
}

impl Ship {
    pub fn new(heading: f32, length: f32, width: f32) -> Self {
        Ship {
            heading: heading,
            length: length,
            width: width,
        }
    }
}

impl Component for Ship {
    // TODO: Is this the wrong storage type? Use something sparser?
    type Storage = VecStorage<Self>;
}

pub fn make_ship<B>(
    builder: B,
    heading: f32,
    length: f32,
    width: f32,
    x: f32,
    y: f32,
    speed: f32,
    bearing: f32,
    angular_velocity: f32,
    _collision_world: &mut CollisionWorld<f32, specs::world::Index>,
) where
    B: specs::world::Builder,
{
    let transform = Transform(Isometry2::new(Vector2::<f32>::new(x, y), 0.0f32));

    // let mut collision_groups = CollisionGroups::new();
    // collision_groups.set_membership(&[ROID_GROUP]);
    // collision_groups.set_whitelist(&[SHIP_GROUP, WEAPON_GROUP]);

    // let collision_isometry = Isometry2::new(Vector2::new(x, y), zero());

    // let collision_shape = ShapeHandle::new(Ball::new(radius));

    // Put entry in collision world
    // let (collision_handle, obj) = collision_world.add(
    //     collision_isometry,
    //     collision_shape,
    //     collision_groups,
    //     GeometricQueryType::Contacts(0.0, 0.0),
    //     0,
    // );

    // Create the entity
    let _entity = builder
        .with(LinearVelocity(from_speed_and_bearing(speed, bearing)))
        .with(AngularVelocity(angular_velocity))
        .with(transform)
        .with(Wrapping)
        // .with(CollisionHandle(collision_handle))
        .with(Ship::new(heading, length, width))
        .build();

    // Annotate the collision object with the entity's ID
    // *obj.data_mut() = entity.id();
}
