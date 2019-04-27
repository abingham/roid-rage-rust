use crate::collide::collide;
use crate::field::Field;
use crate::objects::bullet::Bullet;
use crate::objects::fragment::Fragment;
use crate::objects::roid::Roid;
use crate::objects::game_object::GameObject;
use crate::explode::explode;
use crate::util::{make_velocity_vector, random_bearing};
use nalgebra::Point2;
use opengl_graphics::GlGraphics;
use piston::input::*;
use std::collections::HashSet;

pub struct App {
    pub field: Field,
    pub roids: Vec<Roid>,
    pub bullets: Vec<Bullet>,
    pub fragments: Vec<Fragment>,

    pub full_time: f64,
}

impl App {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let roids = &self.roids;
        let bullets = &self.bullets;
        let fragments = &self.fragments;
        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            for roid in roids {
                roid.render(&WHITE, c, gl);
            }

            for bullet in bullets {
                bullet.render(&WHITE, c, gl)
            }
            
            for fragment in fragments {
                fragment.render(&WHITE, c, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Find all upcoming collisions
        let collisions = collide(&self.roids, &self.bullets, args.dt).iter().fold(
            HashSet::new(),
            |mut acc, x| {
                acc.insert(x.0);
                acc.insert(x.1);
                acc
            },
        );

        // Move all of the roids.
        for roid in &mut self.roids {
            roid.update(args.dt);
            roid.set_position(self.field.wrap(&roid.position()));
            if collisions.contains(&roid.id()) {
                roid.kill();
                self.fragments.extend(explode(roid.position()));
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

        // move all of the fragments
        for fragment in &mut self.fragments {
            fragment.update(args.dt);
        }

        self.roids.retain(|r| r.alive());
        self.bullets.retain(|b| b.alive());
        self.fragments.retain(|f| f.alive());

        self.fire(args.dt);
    }

    fn fire(&mut self, dt: f64) -> () {
        // Generate a bullet if it's the right time.
        self.full_time += dt;
        if self.full_time > 1.0 {
            self.full_time = 0.0;

            let b = Bullet::new(
                Point2::new(
                    (self.field.width() / 2) as f64,
                    (self.field.height() / 2) as f64,
                ),
                make_velocity_vector(200.0,  random_bearing()),
            );
            self.bullets.push(b);
        }
    }
}
