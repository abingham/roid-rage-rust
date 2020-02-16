use piston::input::RenderArgs;
use glutin_window::GlutinWindow as Window;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use opengl_graphics::{GlGraphics, OpenGL};
use crate::controller::Controller;
use crate::model::objects::roid::Roid;
// use crate::model::objects::bullet::Bullet;
// use crate::model::objects::fragment::Fragment;
use crate::model::traits::*;
use graphics;

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
                self.render_objects(&mut gl, args);
            }

            if let Some(args) = e.update_args() {
                self.controller.update(args.dt);
            }
        }
    }
    
    fn render_objects(&self, gl: &mut GlGraphics, args: RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            graphics::clear(BLACK, gl);

            for roid in self.controller.model().roids() {
                roid.render(c, gl);
            }

            // for bullet in &self.controller.model().objects.bullets {
            //     bullet.render(c, gl);
            // }

            // for fragment in &self.controller.model().objects.fragments {
            //     fragment.render(c, gl);
            // }
        });
    }
}

trait Renderable {
    fn render(&self, c: graphics::Context, gl: &mut GlGraphics);
}

impl Renderable for Roid {
    fn render(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position().coords[0], self.position().coords[1]);

        let rect = rectangle::square(-1.0 * self.radius(), -1.0 * self.radius(), 2.0 * self.radius());
        ellipse(*self.color(), rect, transform, gl);
    }
}

// impl Renderable for Bullet {
//    fn render(&self, c: graphics::Context, gl: &mut GlGraphics) {
//         use graphics::*;

//         let transform = c
//             .transform
//             .trans(self.position().coords[0], self.position().coords[1]);

//         let rect = rectangle::square(
//             -1.0 * Bullet::radius(),
//             -1.0 * Bullet::radius(),
//             2.0 * Bullet::radius(),
//         );
//         ellipse([1.0, 1.0, 1.0, 1.0], rect, transform, gl);
//     }
// }

// impl Renderable for Fragment {
//     fn render(&self, c: graphics::Context, gl: &mut GlGraphics) {
//         use graphics::*;

//         let transform = c
//             .transform
//             .trans(self.position().coords[0], self.position().coords[1]);

//         let rect = rectangle::square(
//             -1.0 * Fragment::radius(),
//             -1.0 * Fragment::radius(),
//             2.0 * Fragment::radius(),
//         );
//         ellipse([1.0, 1.0, 1.0, 1.0], rect, transform, gl);
//     }
// }