extern crate glutin_window;
extern crate graphics;
extern crate nalgebra;
extern crate opengl_graphics;
extern crate piston;

use crate::field::Field;
use nalgebra::geometry::Rotation2;
use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use piston::input::*;

pub struct Roid {
    pub position: Point2<f64>,
    pub radius: f64,
    pub speed: f64,
    pub bearing: f64,
}

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub field: Field,
    pub roids: Vec<Roid>,
}

fn move_point(point: &Point2<f64>, distance: f64, bearing: f64) -> Point2<f64> {
    let rot = Rotation2::new(bearing);
    let vec = rot.transform_vector(&Vector2::new(distance, 0.0));
    point + vec
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });

        for roid in &self.roids {
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform.trans(
                    roid.position.coords[0],
                    roid.position.coords[1],
                );

                let rect = rectangle::square(
                    -1.0 * roid.radius,
                    -1.0 * roid.radius,
                    2.0 * roid.radius,
                );
                ellipse(WHITE, rect, transform, gl);
            });
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Move all of the roids.
        for roid in &mut self.roids {
            let point = move_point(&roid.position, roid.speed * args.dt, roid.bearing);
            let point = self.field.wrap(&point);
            roid.position = point;
        }
    }
}
