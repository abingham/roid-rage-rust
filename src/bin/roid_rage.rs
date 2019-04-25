extern crate roid_rage;

use glutin_window::GlutinWindow as Window;
use nalgebra::Point2;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use roid_rage::app::App;
use roid_rage::objects::roid::Roid;
use roid_rage::field::Field;
use roid_rage::util::{make_velocity_vector, random_bearing};
use rand::prelude::*;

fn some_roids(width: usize, height: usize) -> Vec<Roid> {
    let mut rng = thread_rng();
    (0..20).map(|_| {
        Roid::new(
            Point2::new(rng.gen_range(0, width) as f64, 
                        rng.gen_range(0, height) as f64),
            40.0,
            make_velocity_vector(100.0, random_bearing()))
    }).collect()
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Roid Rage!", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        field: Field::new(800, 600, 100),
        roids: some_roids(800, 600),
        bullets: vec![],
        fragments: vec![],
        full_time: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
