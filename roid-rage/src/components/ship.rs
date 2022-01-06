use crate::components::{AngularVelocity, LinearVelocity, Position, Rotation, Wrapping};
use glam::Vec2;
use specs::{Component, HashMapStorage};

pub struct Cannon {
    pub bullet_speed: f32,
    pub rate_of_fire: f32,
}

pub struct Ship {
    pub length: f32,
    pub width: f32,
    pub mass: f32,
    pub thrust: f32,
    pub rotational_speed: f32,
    pub cannon: Cannon,
}

impl Ship {
    pub fn new(
        length: f32,
        width: f32,
        mass: f32,
        thrust: f32,
        rotational_speed: f32,
        cannon: Cannon,
    ) -> Self {
        Ship {
            length: length,
            width: width,
            mass: mass,
            thrust: thrust,
            rotational_speed: rotational_speed,
            cannon: cannon,
        }
    }
}

impl Component for Ship {
    type Storage = HashMapStorage<Self>;
}

pub fn make_ship<B>(
    builder: B,
    heading: f32,
    length: f32,
    width: f32,
    mass: f32,
    thrust: f32,
    rotational_speed: f32,
    cannon: Cannon,
    x: f32,
    y: f32,
    velocity: Vec2,
) -> B
where
    B: specs::world::Builder,
{
    let position = Position(Vec2::new(x, y));
    let rotation = Rotation(heading);

    // Create the entity
    builder
        .with(LinearVelocity(velocity))
        .with(AngularVelocity(0.0))
        .with(position)
        .with(rotation)
        .with(Wrapping)
        .with(Ship::new(
            length,
            width,
            mass,
            thrust,
            rotational_speed,
            cannon,
        ))
}
