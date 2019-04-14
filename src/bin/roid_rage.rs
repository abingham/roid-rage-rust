extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate roid_rage;

use glutin_window::GlutinWindow as Window;
use nalgebra::Point2;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use roid_rage::app::App;
use roid_rage::objects::Circle;
use roid_rage::field::Field;

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
            Circle {
                position: Point2::new(400.0, 300.0),
                radius: 40.0,
                speed: 100.0,
                bearing: 0.0,
            },
            Circle {
                position: Point2::new(400.0, 300.0),
                radius: 40.0,
                speed: 100.0,
                bearing: 2.7,
            },
        ],
        bullets: vec![],
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
