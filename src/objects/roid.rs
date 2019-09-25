use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;

use crate::explode::explode;
use crate::util::{make_velocity_vector, random_bearing};
use crate::collide::Collidable;
use crate::game_object::GameObject;
use crate::util::project;
use crate::field::Field;
use crate::object_set::ObjectSet;
use crate::util::speed;
use uuid;

const MIN_RADIUS: f64 = 10.0;

#[derive(Debug)]
pub struct Roid {
    radius: f64,
    collision_shape: Ball<f64>,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: uuid::Uuid,
    alive: bool
}

impl Roid {
    pub fn new(position: Point2<f64>, radius: f64, velocity: Vector2<f64>) -> Roid {
        Roid {
            position: position,
            velocity: velocity,
            radius: radius,
            collision_shape: Ball::new(radius),
            id: uuid::Uuid::new_v4(),
            alive: true
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl GameObject for Roid {
    fn id(&self) -> uuid::Uuid { self.id }
    fn alive(&self) -> bool { self.alive }
    fn kill(&mut self) -> ObjectSet { 
        self.alive = false; 
                let new_radius = self.radius / 2.0;
        let num_sub_roids = if new_radius >= MIN_RADIUS { 2 } else { 0 };
        let roids = (0..num_sub_roids).map(|_| {
                let velocity = make_velocity_vector(speed((self as &mut dyn GameObject).velocity()) * 2.0, random_bearing());
                Roid::new(self.position, new_radius, velocity)
            })
            .collect();

        ObjectSet::from_objects(roids, vec![], explode(&self.position))
    }

    fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }

    fn update(&mut self, field: &Field, time_delta: f64) {
        self.position = field.wrap(&project(self, time_delta));
    }

    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);
    }
}

impl Collidable for Roid {
    fn collision_shape(&self) -> &dyn Shape<f64> {
        &self.collision_shape
    }

    fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }
}
