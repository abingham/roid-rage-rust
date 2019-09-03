use crate::collide::collide;
use crate::field::Field;
use crate::objects::bullet::Bullet;
use crate::objects::categories::Category;
use crate::objects::game_object::GameObject;
use crate::util::{make_velocity_vector};
use opengl_graphics::GlGraphics;
use piston::input::*;
use std::collections::HashSet;

pub struct App {
    pub field: Field,
    pub objects: Vec<(Category, Box<dyn GameObject>)>,

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
            let roids: Vec<&dyn GameObject> = self
                .objects
                .iter()
                .filter_map(|(c, o)| match c {
                    Category::Roid => Some(o.as_ref()),
                    _ => None,
                })
                .collect();

            let bullets: Vec<&dyn GameObject> = self
                .objects
                .iter()
                .filter_map(|(c, o)| match c {
                    Category::Bullet => Some(o.as_ref()),
                    _ => None,
                })
                .collect();

            // TODO: Collide roids and ships

            // Find all upcoming collisions
            collide(&roids, &bullets, args.dt)
                .iter()
                .fold(HashSet::new(), |mut acc, x| {
                    acc.insert(x.0);
                    acc.insert(x.1);
                    acc
                })
        };

        // Move all objects
        for (_c, obj) in &mut self.objects {
            obj.update(args.dt);
        }

        // Wrap all roids
        for (_, roid) in self
            .objects
            .iter_mut()
            .filter(|(c, _)| c == &Category::Roid)
        {
            roid.set_position(self.field.wrap(&roid.position()));
        }

        let mut new_objects: Vec<(Category, Box<dyn GameObject>)> = vec![];

        // kill collisions
        for (_, obj) in &mut self.objects {
            if collisions.contains(&obj.id()) {
                new_objects.extend(obj.kill());
            }
        }

        // Remove out-of-bounds objects
        for (_, bullet) in &mut self
            .objects
            .iter_mut()
            .filter(|(c, _)| c == &Category::Bullet)
        {
            if !self.field.contains(bullet.position()) {
                new_objects.extend(bullet.kill());
            }
        }

        // Remove all dead objects
        self.objects.retain(|(_c, o)| o.alive());

        self.objects.extend(new_objects);

        self.fire(args.dt);
    }

    fn fire(&mut self, dt: f64) -> () {
        // Generate a bullet if it's the right time.
        self.full_time += dt;
        if self.full_time > 1.0 {
            self.full_time = 0.0;

            let bullets: Vec<Box<dyn GameObject>> = self
                .objects
                .iter()
                .filter_map(|(c, o)| match c {
                    Category::Ship => Some(o.as_ref()),
                    _ => None,
                })
                .map(|s| {
                    let b: Box<dyn GameObject> = Box::new(Bullet::new(
                        s.position().clone(),
                        make_velocity_vector(200.0, /* ship.heading */ 0.0),
                    ));
                    b
                 })
                .collect();

            for b in bullets {
                self.objects.push((Category::Bullet, b));
            }
        }
    }
}
