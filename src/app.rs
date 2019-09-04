use crate::collide::collide;
use crate::objects::bullet::Bullet;
use crate::traits::Moving;
use crate::util::make_velocity_vector;
use crate::object_set::ObjectSet;
use crate::field::Field;
use nalgebra::Point2;
use opengl_graphics::GlGraphics;
use piston::input::*;
use std::collections::HashSet;
use uuid::Uuid;

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

            for roid in self.objects.renderables() {
                roid.render(&WHITE, c, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let (roid_hits, bullet_hits) = {
            let hits = collide(&self.objects.roids, &self.objects.bullets, args.dt);
            let roid_hits: HashSet<Uuid> = hits.iter().fold(HashSet::new(), |mut acc, x| {
                acc.insert(x.0.id());
                acc
            });
            let bullet_hits: HashSet<Uuid> = hits.iter().fold(HashSet::new(), |mut acc, x| {
                acc.insert(x.1.id());
                acc
            });

            (roid_hits, bullet_hits)
        };

        let field = self.field;

        // Update all objects
        for roid in self.objects.updateables() {
            roid.update(&field, args.dt);
        }

        // Explode roids
        // self.state.roids
        //     .iter()
        //     .filter(|r| roid_hits.contains(r.id()))
        //     .map(|r| r);

        // kill collisions
        self.objects.roids.retain(|r| !roid_hits.contains(&r.id()));
        self.objects.bullets.retain(|b| !bullet_hits.contains(&b.id()));

        // Remove out-of-bounds objects
        self.objects.bullets.retain(|b| {
            let p = b.position();
            field.contains(p)
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
