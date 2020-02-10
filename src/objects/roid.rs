use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape, ShapeHandle};
use opengl_graphics::GlGraphics;

// use crate::collide::Collidable;
// use crate::explosion::make_explosion;
use crate::field::Field;
use crate::game_object::GameObject;
use crate::object_set::ObjectSet;
use crate::velocity::{make_velocity_vector, random_bearing, Velocity};
use uuid;

const MIN_RADIUS: f64 = 10.0;

pub struct Roid {
    radius: f64,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: uuid::Uuid,
}

impl Roid {
    pub fn new(position: Point2<f64>, radius: f64, velocity: Vector2<f64>) -> Roid {
        Roid {
            position: position,
            velocity: velocity,
            radius: radius,
            id: uuid::Uuid::new_v4(),
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl GameObject for Roid {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    // fn explode(&self) -> ObjectSet {
    //     let new_radius = self.radius / 2.0;
    //     let num_sub_roids = if new_radius >= MIN_RADIUS { 2 } else { 0 };
    //     let roids = (0..num_sub_roids)
    //         .map(|_| {
    //             let velocity = make_velocity_vector(self.velocity.speed() * 2.0, random_bearing());
    //             Roid::new(self.position, new_radius, velocity)
    //         })
    //         .collect();

    //     ObjectSet::from_objects(roids, vec![], make_explosion(&self.position))
    // }

    fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }

    fn update(&mut self, field: &Field, time_delta: f64) -> () {
        let new_position = field.wrap(&(self.position + self.velocity * time_delta));
        self.position = new_position;
        // self.collision_shape
   }

    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);
    }

    fn collision_shape(&self) -> ShapeHandle<f64> {
        ShapeHandle::new(Ball::new(self.radius))
    }
}

