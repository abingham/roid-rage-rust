use crate::components::{AngularVelocity, LinearVelocity, Position, Rotation, Wrapping};
use sted::Bearing;
use glam::Vec2;
use crate::core::util::from_speed_and_bearing;
use ncollide2d::world::CollisionWorld;
use specs::{Component, HashMapStorage};

pub struct Ship {
    pub length: f32,
    pub width: f32,
    pub mass: f32,
    pub thrust: f32
}

impl Ship {
    pub fn new(length: f32, width: f32, mass: f32, thrust: f32) -> Self {
        Ship {
            length: length,
            width: width,
            mass: mass,
            thrust: thrust
        }
    }
}

impl Component for Ship {
    type Storage = HashMapStorage<Self>;
}

pub fn make_ship<B>(
    builder: B,
    heading: Bearing<f32>,
    length: f32,
    width: f32,
    mass: f32,
    thrust: f32,
    x: f32,
    y: f32,
    speed: f32,
    course: Bearing<f32>,
    angular_velocity: f32,
    _collision_world: &mut CollisionWorld<f32, specs::world::Index>,
) where
    B: specs::world::Builder,
{
    let position = Position(Vec2::new(x, y));
    let rotation = Rotation(heading);

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
        .with(LinearVelocity(from_speed_and_bearing(
            speed,
            course.radians(),
        )))
        .with(AngularVelocity(angular_velocity))
        .with(position)
        .with(rotation)
        .with(Wrapping)
        // .with(CollisionHandle(collision_handle))
        .with(Ship::new(length, width, mass, thrust))
        .build();

    // Annotate the collision object with the entity's ID
    // *obj.data_mut() = entity.id();
}
