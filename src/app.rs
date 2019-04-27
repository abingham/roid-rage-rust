use crate::collide::collide;
use crate::field::Field;
use crate::objects::bullet::Bullet;
use crate::objects::fragment::Fragment;
use crate::objects::roid::Roid;
use crate::objects::categories::Category;
use crate::objects::game_object::GameObject;
use crate::explode::explode;
use crate::util::{make_velocity_vector, random_bearing};
use nalgebra::Point2;
use opengl_graphics::GlGraphics;
use piston::input::*;
use std::collections::HashSet;

pub struct App {
    pub field: Field,
    pub objects: Vec<(Category, Box<GameObject>)>,

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

            for (_c, obj) in &self.objects {
                obj.render(&WHITE, c, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let collisions = {
            let roids: Vec<&GameObject> = self.objects.iter()
                .filter_map(|(c, o)| match c {
                    Category::Roid => Some(o.as_ref()),
                    _ => None
                })
                .collect();

            let bullets: Vec<&GameObject> = self.objects.iter()
                .filter_map(|(c, o)| match c {
                    Category::Bullet => Some(o.as_ref()),
                    _ => None
                })
                .collect();

            // Find all upcoming collisions
            collide(&roids, &bullets, args.dt).iter().fold(
                HashSet::new(),
                |mut acc, x| {
                    acc.insert(x.0);
                    acc.insert(x.1);
                    acc
                },
            )
        };

        // Move all objects
        for (_c, obj) in &mut self.objects {
            obj.update(args.dt);
        }

        // Wrap all roids
        for (_, roid) in self.objects.iter_mut().filter(|(c, _)| c == &Category::Roid) {
            roid.set_position(self.field.wrap(&roid.position()));
        }

        // kill collisions
        for (_, obj) in &mut self.objects {
            if collisions.contains(&obj.id()) {
                obj.kill(); // TODO: Let this return a vector of new objects.
            }
        }

        // Remove out-of-bounds objects
        for (_, bullet) in &mut self.objects.iter_mut().filter(|(c, _)| c == &Category::Bullet) {
            if !self.field.contains(bullet.position()) {
                bullet.kill();
            }
        }

        // Remove all dead objects
        self.objects.retain(|(_c, o)| o.alive());

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
            self.objects.push((Category::Bullet, Box::new(b)));
        }
    }
}
