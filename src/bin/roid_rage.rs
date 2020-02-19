extern crate roid_rage;

// use rand::prelude::*;
// use roid_rage::controller::basic_controller::BasicController;
// use roid_rage::model::field::Field;
// use roid_rage::model::object_set::ObjectSet;
// use roid_rage::model::objects::roid::Roid;
// use roid_rage::model::Model;
// use roid_rage::velocity::{make_velocity_vector, random_bearing};
use specs::{DispatcherBuilder, Join, Builder, WriteStorage, ReadStorage, System, World, WorldExt, RunNow};
use roid_rage::components::{Position, Velocity};
// use roid_rage::view::View;

// fn some_roids(width: usize, height: usize) -> Vec<Roid> {
//     let mut rng = thread_rng();
//     (1..10)
//         .map(|_| {
//             Roid::new(
//                 Point2::new(
//                     rng.gen_range(0, width) as f64,
//                     rng.gen_range(0, height) as f64,
//                 ),
//                 40.0,
//                 make_velocity_vector(100.0, random_bearing()),
//             )
//         })
//         .collect()
// }

// fn the_ship(width: usize, height: usize) -> (Category, Box<dyn GameObject>) {
//     (Category::Ship,
//      Box::new(
//         Ship::new(
//             Point2::new((width / 2) as f64, (height / 2) as f64),
//             make_velocity_vector(0.0, 0.0),
//             0.0)))
// }

struct RoidRage;

impl<'a> System<'a> for RoidRage {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (pos, vel): Self::SystemData) {
        for (pos, vel) in (&pos, &vel).join() {
            println!("pos={} vel={}", pos.pos, vel.vel)
        }
    }
}

struct UpdatePositions;

impl<'a> System<'a> for UpdatePositions {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            // TODO: 0.05 is our fake time delta. We need to get this from somewhere! See tutorial.
            pos.pos = pos.pos + vel.vel * 0.05;
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.create_entity()
        .with(Position::new(400.0, 300.0))
        .with(Velocity::from_speed_bearing(1.0, 0.0))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(RoidRage, "roid_rage", &[])
        .with(UpdatePositions, "update_positions", &["roid_rage"])
        .with(RoidRage, "roid_rage_updated", &["update_positions"])
        .build();

    dispatcher.dispatch(&mut world);
    world.maintain();
}
