extern crate roid_rage;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::prelude::*;
use roid_rage::app::App;
use roid_rage::field::Field;
use roid_rage::objects::categories::Category;
use roid_rage::objects::game_object::GameObject;
use roid_rage::objects::roid::Roid;
use roid_rage::objects::ship::Ship;
use roid_rage::objects::bullet::Bullet;
use roid_rage::collide::{Collidable, collide};
use std::iter::IntoIterator;
use ncollide2d::shape::Shape;
use nalgebra::{Point2, Vector2};

use roid_rage::util::{make_velocity_vector, random_bearing};

// fn some_roids(width: usize, height: usize) -> Vec<(Category, Box<dyn GameObject>)> {
//     let mut rng = thread_rng();
//     let mut result: Vec<(Category, Box<dyn GameObject>)> = vec![];
//     for _ in 1..3 {
//         let roid = Roid::new(
//             Point2::new(
//                 rng.gen_range(0, width) as f64,
//                 rng.gen_range(0, height) as f64,
//             ),
//             40.0,
//             make_velocity_vector(100.0, random_bearing()),
//         );
//         result.push((Category::Roid, Box::new(roid)));
//     }
//     result
// }

fn some_roids(width: usize, height: usize) -> Vec<Roid> {
    let mut rng = thread_rng();
    let mut result: Vec<Roid> = vec![];
    // TODO: Can we map over range 1-3?
    for _ in 1..3 {
        let roid = Roid::new(
            Point2::new(
                rng.gen_range(0, width) as f64,
                rng.gen_range(0, height) as f64,
            ),
            40.0,
            make_velocity_vector(100.0, random_bearing()),
        );
        result.push(roid);
    }
    result
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
    let roids1 = some_roids(800, 600);

    let bullets: Vec<Bullet> = vec![];

    let colls = collide(&roids1, &bullets, 0.5);
}

// fn main() {
//     // Change this to OpenGL::V2_1 if not working.
//     let opengl = OpenGL::V3_2;

//     // Create an Glutin window.
//     let mut window: Window = WindowSettings::new("Roid Rage!", [800, 600])
//         .opengl(opengl)
//         .exit_on_esc(true)
//         .build()
//         .unwrap();

//     let mut objects = some_roids(800, 600);
//     objects.push(the_ship(800, 600));

//     // Create a new game and run it.
//     let mut app = App {
//         field: Field::new(800, 600, 100),
//         objects: objects,
//         full_time: 0.0,
//     };

//     let mut gl = GlGraphics::new(opengl);
//     let mut events = Events::new(EventSettings::new());
//     while let Some(e) = events.next(&mut window) {
//         if let Some(r) = e.render_args() {
//             app.render(&mut gl, &r);
//         }

//         if let Some(u) = e.update_args() {
//             app.update(&u);
//         }
//     }
// }
