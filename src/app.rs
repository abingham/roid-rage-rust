use crate::collide::collide;
use crate::field::Field;
use crate::object_set::ObjectSet;
use crate::objects::bullet::Bullet;
use crate::game_object::GameObject;
use crate::util::make_velocity_vector;
use nalgebra::Point2;
use opengl_graphics::GlGraphics;
use piston::input::*;
use std::collections::HashSet;

pub struct App {
    pub field: Field,
    pub objects: ObjectSet,
    pub full_time: f64,
}

impl App {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            for roid in self.objects.iter() {
                roid.render(&WHITE, c, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let hits = collide(&self.objects.roids, &self.objects.bullets, args.dt)
            .iter()
            .fold(HashSet::new(), |mut acc, x| {
                acc.insert(x.0.id());
                acc.insert(x.1.id());
                acc
            });

        let field = self.field;

        // Update all objects
        for roid in self.objects.iter_mut() {
            roid.update(&field, args.dt);
        }

        // Explode collisions 
        let updates: Vec<ObjectSet> = self
            .objects
            .iter_mut()
            .filter_map(|r| {
                if hits.contains(&r.id()) {
                    Some(r.kill())
                } else {
                    None
                }
            })
            .collect();

        // Kill out-of-bounds objects
        for bullet in &mut self.objects.bullets {
            if !field.contains(bullet.position()) {
                bullet.kill();
            }
        }

        // kill collisions
        self.objects.remove_dead();

        // Insert new objects
        updates.into_iter().fold(&mut self.objects, |acc, x| {
            acc.extend(x);
            acc
        });

        self.fire(args.dt);
    }

    fn fire(&mut self, dt: f64) -> () {
        // Generate a bullet if it's the right time.
        self.full_time += dt;
        if self.full_time > 1.0 {
            self.full_time = 0.0;

            let bullet = Bullet::new(
                Point2::new(
                    (self.field.width() / 2) as f64,
                    (self.field.height() / 2) as f64,
                ),
                make_velocity_vector(200.0, 0.0),
            );
            self.objects.bullets.push(bullet);
        }
    }
}
