use crate::collide::collision_vector;
use crate::field::Field;
use crate::object_set::ObjectSet;
use crate::objects::bullet::Bullet;
use crate::velocity::{make_velocity_vector, Velocity};
use nalgebra::Point2;
use opengl_graphics::GlGraphics;
use piston::input::*;

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
        // See what got hit
        let hits = self.objects.collisions(args.dt);

        // Update or explode everything.
        self.objects = self
            .objects
            .iter()
            .filter_map(|o| {
                // If an object was hit, its debris goes into the new object set
                if hits.contains(&o.id()) {
                    Some(o.explode())
                }
                // otherwise, the object's update goes into the new object set
                else {
                    Some(o.update(&self.field, args.dt))
                }
            })
            .fold(ObjectSet::new(), |mut acc, s| {
                acc.extend(s);
                acc
            });

        self.fire(args.dt);
    }

    fn fire(&mut self, dt: f64) -> () {
        // Generate a bullet if it's the right time.
        self.full_time += dt;
        if self.full_time > 1.0 {
            let hit = self
                .objects
                .roids()
                .filter_map(|roid| {
                    collision_vector(
                        &Point2::new(
                            (self.field.width() / 2) as f64,
                            (self.field.height() / 2) as f64,
                        ),
                        200.0,
                        roid,
                    )
                })
                .filter(|(p, _v)| self.field.contains(p))
                .nth(0);

            match hit {
                Some((_p, v)) => {
                    self.full_time = 0.0;

                    let bullet = Bullet::new(
                        Point2::new(
                            (self.field.width() / 2) as f64,
                            (self.field.height() / 2) as f64,
                        ),
                        make_velocity_vector(200.0, v.bearing()),
                    );

                    let bullets = ObjectSet::from_objects(vec![], vec![bullet], vec![]);
                    self.objects.extend(bullets);
                }
                None => {}
            }
        }
    }
}
