extern crate glutin_window;
extern crate graphics;
extern crate nalgebra;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use crate::field::Field;
use crate::objects::Circle;
use crate::util::make_velocity_vector;
use nalgebra::geometry::Isometry2;
use nalgebra::{Point2, Vector2};
use ncollide2d::query;
use ncollide2d::shape::Ball;
use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::prelude::*;
use std::collections::HashSet;
use std::f64::consts::PI;
use uuid::Uuid;

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
        let collisions = self.collide(args.dt);

        // Move all of the roids.
        for roid in &mut self.roids {
            roid.update(args.dt);
            roid.set_position(self.field.wrap(&roid.position()));
        }

        // move the bullets
        for bullet in &mut self.bullets {
            bullet.update(args.dt);
        }

        let f = &self.field;
        self.roids.retain(|r| !collisions.contains(&r.id()));
        self.bullets.retain(|b| !collisions.contains(&b.id()));
        self.bullets.retain(|b| f.contains(b.position()));

        self.fire(args.dt);
    }

    fn collide(&mut self, dt: f64) -> HashSet<Uuid> {
        // Find bullet/roid intersections
        // let mut dead_objects: HashSet<Uuid> = HashSet::new();
        // let tois: HashSet<Uuid> = self.roids.iter()
        self.roids
            .iter()
            .map(|roid| {
                let roid_ball = Ball::new(roid.radius());
                let roid_pos = Isometry2::new(
                    Vector2::new(roid.position().coords[0], roid.position().coords[1]),
                    0.0,
                );
                let foo: Vec<Uuid> = self
                    .bullets
                    .iter()
                    .filter_map(|bullet| {
                        let bullet_ball = Ball::new(bullet.radius());
                        let bullet_pos = Isometry2::new(
                            Vector2::new(bullet.position().coords[0], bullet.position().coords[1]),
                            0.0,
                        );
                        let toi = query::time_of_impact(
                            &roid_pos,
                            roid.velocity(),
                            &roid_ball,
                            &bullet_pos,
                            bullet.velocity(),
                            &bullet_ball,
                        );
                        match toi {
                            Some(t) => {
                                if t <= dt {
                                    Some(vec![roid.id(), bullet.id()])
                                } else {
                                    None
                                }
                            },
                            None => None,
                        }
                    })
                    .flatten()
                    .collect();
                foo
            })
            .flatten()
            .collect()
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
