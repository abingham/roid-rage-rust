extern crate glutin_window;
extern crate graphics;
extern crate nalgebra;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use crate::field::Field;
use crate::objects::{Circle, GameObject};
use crate::util::make_velocity_vector;
use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::prelude::*;
use std::collections::HashSet;
use std::f64::consts::PI;
use crate::collide::collide;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub field: Field,
    pub roids: Vec<Circle>,
    pub bullets: Vec<Circle>,
    pub fragments: Vec<Circle>,

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
        // Find all upcoming collisions
        let collisions = collide(&self.roids, &self.bullets, args.dt)
            .iter()
            .fold(HashSet::new(), |mut acc, x| {
                acc.insert(x.0);
                acc.insert(x.1);
                acc
            });

        // Move all of the roids.
        for roid in &mut self.roids {
            roid.update(args.dt);
            roid.set_position(self.field.wrap(&roid.position()));
            if collisions.contains(&roid.id()) {
                roid.kill();
            }
        }

        // move the bullets
        for bullet in &mut self.bullets {
            bullet.update(args.dt);
            if !self.field.contains(bullet.position()) {
                bullet.kill();
            } else if collisions.contains(&bullet.id()) {
                bullet.kill();
            }
        }

        self.roids.retain(|r| r.alive());
        self.bullets.retain(|b| b.alive());

        self.fire(args.dt);
    }

    fn fire(&mut self, dt: f64) -> () {
        // Generate a bullet if it's the right time.
        self.full_time += dt;
        if self.full_time > 1.0 {
            self.full_time = 0.0;

            let mut rng = thread_rng();
            let bearing: f64 = rng.gen();

            let b = Circle::new(
                Point2::new(
                    (self.field.width() / 2) as f64,
                    (self.field.height() / 2) as f64,
                ),
                2.0,
                make_velocity_vector(200.0, (bearing * 2.0 - 1.0) * PI),
            );
            self.bullets.push(b);
        }
    }
}
