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
use roid_rage::util::make_velocity_vector;

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
        roids: vec![
            Roid::new(
                Point2::new(400.0, 300.0),
                40.0,
                make_velocity_vector(100.0, 0.0),
            ),
            Roid::new(
                Point2::new(400.0, 300.0),
                40.0,
                make_velocity_vector(100.0, 2.7)
            ),
        ],
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
