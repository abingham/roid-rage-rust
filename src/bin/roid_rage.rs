extern crate roid_rage;

// use rand::prelude::*;
// use roid_rage::controller::basic_controller::BasicController;
// use roid_rage::model::field::Field;
// use roid_rage::model::object_set::ObjectSet;
// use roid_rage::model::objects::roid::Roid;
// use roid_rage::model::Model;
// use roid_rage::velocity::{make_velocity_vector, random_bearing};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::UpdateEvent;
use piston::window::WindowSettings;
use roid_rage::components::{Position, Velocity};
use specs::{
    Builder, DispatcherBuilder, Join, Read, ReadStorage, System, World, WorldExt, WriteStorage,
};
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

#[derive(Default)]
struct DeltaTime(f64);

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

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world
        .create_entity()
        .with(Position::new(400.0, 300.0))
        .with(Velocity::from_speed_bearing(1.0, 0.0))
        .build();
    world.insert(DeltaTime(0.05));

    let mut dispatcher = DispatcherBuilder::new()
        .with(RoidRage, "roid_rage", &[])
        .with(UpdatePositions, "update_positions", &["roid_rage"])
        .with(RoidRage, "roid_rage_updated", &["update_positions"])
        .build();

    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Roid Rage!", (800, 600))
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // if let Some(args) = e.render_args() {
        //     self.render_objects(&mut gl, args);
        // }
        if let Some(args) = e.update_args() {
            {
                let mut delta = world.write_resource::<DeltaTime>();
                *delta = DeltaTime(args.dt);
            }

            dispatcher.dispatch(&mut world);
            world.maintain();
        }
    }

    dispatcher.dispatch(&mut world);
    world.maintain();
}
