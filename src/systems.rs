use crate::components::{CollisionHandle, Position, Velocity};
use nalgebra::{zero, Isometry2, Vector2};
use ncollide2d::world::CollisionWorld;
use specs::{Join, Read, ReadStorage, System, WriteExpect, WriteStorage};

#[derive(Default)]
pub struct DeltaTime(pub f64);

pub struct PositionLogger;

impl<'a> System<'a> for PositionLogger {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (pos, vel): Self::SystemData) {
        for (pos, vel) in (&pos, &vel).join() {
            println!("pos={} vel={}", pos.pos, vel.vel)
        }
    }
}

pub struct UpdatePositions;

impl<'a> System<'a> for UpdatePositions {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut pos, vel, time_delta): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.pos = pos.pos + vel.vel * time_delta.0;
        }
    }
}

#[derive(Default)]
pub struct Collide;

impl<'a> System<'a> for Collide {
    type SystemData = (
        ReadStorage<'a, CollisionHandle>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, CollisionWorld<f64, ()>>,
    );

    fn run(&mut self, (handles, positions, mut world): Self::SystemData) {
        for (handle, pos) in (&handles, &positions).join() {
            if let Some(collision_object) = world.get_mut(handle.0) {
                collision_object
                    .set_position(Isometry2::new(Vector2::new(pos.x(), pos.y()), zero()));
            }
        }
    }
}
