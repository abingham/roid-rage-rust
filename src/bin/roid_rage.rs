extern crate roid_rage;

use glutin_window::GlutinWindow as Window;
use nalgebra::Point2;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::prelude::*;
use roid_rage::app::App;
use roid_rage::field::Field;
use roid_rage::object_set::ObjectSet;
use roid_rage::game_object::GameObject;
use roid_rage::objects::roid::Roid;
use roid_rage::velocity::{make_velocity_vector, random_bearing};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::{CollisionObjectHandle, CollisionGroups, CollisionWorld};
use ncollide2d::world::GeometricQueryType;
use nalgebra as na;
use graphics::*;
use std::collections::HashMap;
use uuid::Uuid;

fn some_roids(width: usize, height: usize) -> Vec<Roid> {
    let mut rng = thread_rng();
    (1..2)
        .map(|_| {
            Roid::new(
                Point2::new(
                    rng.gen_range(0, width) as f64,
                    rng.gen_range(0, height) as f64,
                ),
                40.0,
                make_velocity_vector(100.0, random_bearing()),
            )
        })
        .collect()
}

// fn the_ship(width: usize, height: usize) -> (Category, Box<dyn GameObject>) {
//     (Category::Ship,
//      Box::new(
//         Ship::new(
//             Point2::new((width / 2) as f64, (height / 2) as f64),
//             make_velocity_vector(0.0, 0.0),
//             0.0)))
// }

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;


    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Roid Rage!", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut ball_group = CollisionGroups::new();
    ball_group.set_membership(&[1]);

    let contacts_query  = GeometricQueryType::Contacts(0.0, 0.0);
    let proximity_query = GeometricQueryType::Proximity(0.0);

    let mut world = CollisionWorld::new(0.02);

    let mut roids: HashMap<Uuid, (Roid, CollisionObjectHandle)>;
    for roid in some_roids(800, 600) {
        let ball = roid.collision_shape();
        let ball_pos = na::Isometry2::new(na::Vector2::new(roid.position()[0], roid.position()[1]), na::zero());
        let ball_handle = world.add(ball_pos, ball, ball_group, contacts_query, Some(roid.id()));
        roids.insert(roid.id(), (roid, ball_handle));
    }

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                
                for roid in roids {
                    roid.render(&WHITE, c, gl);
                }
            })
        }

        if let Some(args) = e.update_args() {
            for roid in ball_groups {

            }
            let ball_pos;
    {
        // Integrate the velocities.
        let ball_object   = world.collision_object(ball_handle).unwrap();
        let ball_velocity = ball_object.data.velocity.as_ref().unwrap();

        // Integrate the positions.
        ball_pos = ball_object.position.append_translation(&(timestep * ball_velocity.get()));
    }
        }

    //     if let Some(r) = e.render_args() {
    //         app.render(&mut gl, &r);
    //     }

    //     if let Some(u) = e.update_args() {
    //         app.update(&u);
    //     }
    // }
}
