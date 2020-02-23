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
use roid_rage::components::{CollisionHandle, Position, Velocity};
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use ncollide2d::world::CollisionWorld;
use nalgebra::{Isometry2, Vector2, zero};
use ncollide2d::pipeline::CollisionGroups;
use roid_rage::collision_groups::roid_collision_groups;
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::pipeline::{CollisionObjectSlabHandle, ContactEvent, GeometricQueryType};

use roid_rage::systems::*;

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

fn add_roid(world: &mut World) {
    use ncollide2d::shape::{Ball, ShapeHandle};

    let radius = 10.0;
    let (x, y) = (400.0, 300.0);

    // insert collision shape into global collision world
    let (handle, _) = world.write_resource::<CollisionWorld<f64, ()>>().add(
        Isometry2::new(Vector2::<f64>::new(x, y), zero()),
        ShapeHandle::new(Ball::new(radius)),
        roid_collision_groups(),
        GeometricQueryType::Contacts(0.0, 0.0),
        (),
    );
    
    // Create entity, with the collision handle as one of its components.
    world.create_entity()
        .with(Position::new(x, y))
        .with(Velocity::from_speed_bearing(1.0, 0.0))
        .with(CollisionHandle(handle))
        .build();
}

fn main() {
    let mut world = World::new();
    world.insert(CollisionWorld::<f64, ()>::new(0.02f64));

    let mut dispatcher = DispatcherBuilder::new()
        .with(UpdatePositions, "update_positions", &[])
        .with(Collide, "collide", &["update_positions"])
        .build();

    dispatcher.setup(&mut world);

    add_roid(&mut world);

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
}