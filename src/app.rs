extern crate glutin_window;
extern crate graphics;
extern crate nalgebra;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use crate::field::Field;
use nalgebra::geometry::Isometry2;
use nalgebra::{Point2, Vector2};
use ncollide2d::query;
use ncollide2d::shape::Ball;
use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::prelude::*;
use std::collections::HashSet;
use std::f64::consts::PI;
use crate::objects::Circle;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub field: Field,
    pub roids: Vec<Circle>,
    pub bullets: Vec<Circle>,

    pub full_time: f64,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });

        for roid in &self.roids {
            self.gl.draw(args.viewport(), |c, gl| {
                roid.render(&WHITE, c, gl);
            });
        }

        for bullet in &self.bullets {
            self.gl.draw(args.viewport(), |c, gl| {
                bullet.render(&WHITE, c, gl);
            });
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Move all of the roids.
        for roid in &mut self.roids {
            roid.update(args.dt);
            roid.position = self.field.wrap(&roid.position);
        }

        // move the bullets
        for bullet in &mut self.bullets {
            bullet.update(args.dt);
            // TODO: Remove bullets which are off the field
        }

        // Find bullet/roid intersections
        for bullet in &self.bullets {
            let bullet_ball = Ball::new(bullet.radius);
            let bullet_trans = Isometry2::new(
                Vector2::new(bullet.position.coords[0], bullet.position.coords[1]),
                0.0,
            );

            let mut dead_roids = HashSet::new();
            for (i, roid) in self.roids.iter().enumerate() {
                let roid_ball = Ball::new(roid.radius);
                let roid_trans = Isometry2::new(
                    Vector2::new(roid.position.coords[0], roid.position.coords[1]),
                    0.0,
                );

                let dist_intersecting =
                    query::distance(&bullet_trans, &bullet_ball, &roid_trans, &roid_ball);

                if dist_intersecting == 0.0 {
                    dead_roids.insert(i);
                }
            }
            let mut dead_roids: Vec<&usize> = dead_roids.iter().collect();
            dead_roids.sort();
            dead_roids.reverse();
            for i in dead_roids {
                self.roids.remove(*i);
            }
        }

        // Generate a bullet if it's the right time.
        self.full_time += args.dt;
        if self.full_time > 1.0 {
            self.full_time = 0.0;

            let mut rng = thread_rng();
            let bearing: f64 = rng.gen();

            let b = Circle {
                position: Point2::new(
                    (self.field.width() / 2) as f64,
                    (self.field.height() / 2) as f64,
                ),
                radius: 2.0,
                speed: 200.0,
                bearing: (bearing * 2.0 - 1.0) * PI,
            };
            self.bullets.push(b);
        }
    }
}
