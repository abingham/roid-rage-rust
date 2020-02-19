use crate::components::{CollisionShape, Position, Velocity};
use specs::{BitSet, ReaderId, World, Join, Read, Write, ReadStorage, System, WriteStorage};
use ncollide2d::world::CollisionWorld;
use specs::storage::ComponentEvent;
use specs::shred::SystemData; 
use specs::storage::Storage;

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
pub struct Collide {
    collision_world: Option<CollisionWorld<f64, ()>>,
    pub reader_id: Option<ReaderId<ComponentEvent>>
}

impl<'a> System<'a> for Collide {
    type SystemData = (
        ReadStorage<'a, CollisionShape>,
    );

    fn run(&mut self, _: Self::SystemData) {
        let collision_world = &mut self.collision_world.as_mut().unwrap();
        // TODO: Remove anything from the collision world which is not in the collidable component
        // let removals = ...vector of CollisionHandles to remove...
        // collision_world.remove(&removals);

        collision_world.update();

        // TODO: Process new collisions
    }

    fn setup(&mut self, world: &mut World) {
        // Self::SystemData::setup(world);
        self.collision_world = Some(CollisionWorld::<f64, ()>::new(0.02f64));
        self.reader_id = Some(
            WriteStorage::<CollisionShape>::fetch(&world).register_reader());
    }
}