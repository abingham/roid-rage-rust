use crate::field::Field;
use crate::game_object::GameObject;
use crate::objects::roid::Roid;
use nalgebra as na;
use ncollide2d::pipeline::CollisionGroups;
use ncollide2d::pipeline::CollisionObjectSlabHandle;
use ncollide2d::pipeline::GeometricQueryType;
use ncollide2d::world::CollisionWorld;
use opengl_graphics::GlGraphics;
use piston::input::*;

pub struct App {
    field: Field,
    roids: Vec<(Roid, CollisionObjectSlabHandle)>,
    full_time: f64,
    collision_world: CollisionWorld<f64, Option<()>>,
}

const FIRING_FREQUENCY: f64 = 0.5;

impl App {
    pub fn new(field: Field, roids: Vec<Roid>) -> App {
        let mut world: CollisionWorld<f64, Option<()>> = CollisionWorld::new(0.02f64);
        let contacts_query = GeometricQueryType::Contacts(0.0, 0.0);

        let roids: Vec<(Roid, CollisionObjectSlabHandle)> = roids
            .into_iter()
            .map(|roid| {
                let roid_pos = na::Isometry2::new(
                    na::Vector2::new(roid.position()[0], roid.position()[1]),
                    na::zero(),
                );
                let (handle, _obj) = world.add(
                    roid_pos, 
                    roid.collision_shape(),
                    roid.collision_groups(),
                    contacts_query, 
                    None);
                (roid, handle)
            })
            .collect();

        App {
            field: field,
            roids: roids,
            full_time: 0.0,
            collision_world: world,
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for (roid, _) in &self.roids {
                roid.render(&WHITE, c, gl);
            }
        });
    }

    pub fn update(&mut self, time_delta: f64) {
        self.full_time += time_delta;

        for (roid, handle) in &mut self.roids {
            roid.update(&self.field, time_delta);
            if let Some(object) = self.collision_world.get_mut(*handle) {
                let pos = na::Isometry2::new(
                    na::Vector2::new(roid.position()[0], roid.position()[1]),
                    na::zero(),
                );
                object.set_position(pos);
            }
        }
        self.collision_world.update();
    }

    // fn fire(&mut self, dt: f64) -> () {
    //     let firing_position = Point2::new(
    //         (self.field.width() / 2) as f64,
    //         (self.field.height() / 2) as f64,
    //     );

    //     // Generate a bullet if it's the right time.
    //     self.full_time += dt;
    //     if self.full_time > FIRING_FREQUENCY {
    //         match target(
    //             &firing_position,
    //             Bullet::speed(),
    //             &self.field,
    //             &self.objects,
    //         ) {
    //             Some(bearing) => {
    //                 self.full_time = 0.0;

    //                 let bullet = Bullet::new(
    //                     firing_position,
    //                     make_velocity_vector(Bullet::speed(), bearing),
    //                 );

    //                 let bullets = ObjectSet::from_objects(vec![], vec![bullet], vec![]);
    //                 self.objects.extend(bullets);
    //             }
    //             None => {}
    //         }
    //     }
    // }
}
