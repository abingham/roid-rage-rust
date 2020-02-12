use glutin_window::GlutinWindow as Window;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use opengl_graphics::{GlGraphics, OpenGL};
use crate::controller::Controller;

// TODO: Do we need a view trait? Or is this enough for now?
pub struct View {
    controller: Box<dyn Controller>,
    size: [u32; 2],
}

impl View {
    pub fn new(controller: Box<dyn Controller>, size: [u32; 2]) -> View {
        View {
            controller: controller,
            size: size,
        }
    }

    pub fn run(&mut self) {
        let opengl = OpenGL::V3_2;
        // Create an Glutin window.
        let mut window: Window = WindowSettings::new("Roid Rage!", self.size)
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();


        let mut gl = GlGraphics::new(opengl);
        let mut events = Events::new(EventSettings::new());

        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                self.controller.model().render(&mut gl, &args);
            }

            if let Some(args) = e.update_args() {
                self.controller.update(args.dt);
            }
        }
    }
}