extern crate glutin_window;
extern crate graphics;
extern crate nalgebra;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use crate::field::Field;
use nalgebra::geometry::Rotation2;
use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::prelude::*;
use std::f64::consts::PI;

pub struct Roid {
    pub position: Point2<f64>,
    pub radius: f64,
    pub speed: f64,
    pub bearing: f64,
}

pub struct Bullet {
    pub position: Point2<f64>,
    pub radius: f64,
    pub speed: f64,
    pub bearing: f64
}

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub field: Field,
    pub roids: Vec<Roid>,
    pub bullets: Vec<Bullet>,

    pub full_time: f64,
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

        for bullet in &self.bullets {
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform.trans(
                    bullet.position.coords[0],
                    bullet.position.coords[1],
                );

                let rect = rectangle::square(
                    -1.0 * bullet.radius,
                    -1.0 * bullet.radius,
                    2.0 * bullet.radius,
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

        // move the bullets
        for bullet in &mut self.bullets {
            let point = move_point(&bullet.position, bullet.speed * args.dt, bullet.bearing);
            bullet.position = point;
            // TODO: Remove bullets which are off the field
        }


        self.full_time += args.dt;
        if self.full_time > 1.0 {
            self.full_time = 0.0;

            let mut rng = thread_rng();
            let bearing: f64 = rng.gen();

            let b = Bullet {
                position: Point2::new((self.field.width() / 2) as f64, 
                                      (self.field.height() / 2) as f64),
                radius: 2.0,
                speed: 200.0,
                bearing: (bearing * 2.0 - 1.0) * PI
            };
            self.bullets.push(b);
        }
    }
}
