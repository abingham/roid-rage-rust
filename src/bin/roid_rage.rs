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
use roid_rage::game_object::GameObject;
use roid_rage::objects::roid::Roid;
use roid_rage::velocity::{make_velocity_vector, random_bearing};

fn some_roids(width: usize, height: usize) -> Vec<Box<dyn GameObject>> {
    let mut rng = thread_rng();
    (1..20)
        .map(|_| {
            let roid = Roid::new(
                Point2::new(
                    rng.gen_range(0, width) as f64,
                    rng.gen_range(0, height) as f64,
                ),
                40.0,
                make_velocity_vector(100.0, random_bearing()),
            );
            Box::new(roid) as Box<dyn GameObject>
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

    let field = Field::new(800, 600, 100);

    let mut app = App::new(field, some_roids(800, 600));

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&mut gl, &args);
        }

        if let Some(args) = e.update_args() {
            app.update(args.dt);
        }
    }
}
