use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;

// use crate::explode::explode;
use crate::util::{make_velocity_vector, random_bearing};
use crate::collide::Collidable;
use crate::traits::{Renderable, Updateable, Moving, Splode};
use crate::util::project;
use crate::field::Field;
use crate::object_set::ObjectSet;
use std::hash::{Hash, Hasher};
use uuid;

const MIN_RADIUS: f64 = 10.0;

#[derive(Debug)]
pub struct Roid {
    radius: f64,
    collision_shape: Ball<f64>,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: uuid::Uuid
}

impl Roid {
    pub fn new(position: Point2<f64>, radius: f64, velocity: Vector2<f64>) -> Roid {
        Roid {
            position: position,
            velocity: velocity,
            radius: radius,
            collision_shape: Ball::new(radius),
            id: uuid::Uuid::new_v4()
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
}

impl PartialEq for Roid {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Roid {}

impl Hash for Roid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Moving for Roid {
    fn position(&self) -> &Point2<f64> {
        &self.position
    }

    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }
}

impl Updateable for Roid {
    fn update(&mut self, field: &Field, time_delta: f64) {
        self.position = field.wrap(&project(self, time_delta));
    }
}

impl Collidable for Roid {
    fn collision_shape(&self) -> &dyn Shape<f64> {
        &self.collision_shape
    }
}

impl Renderable for Roid {
    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);
    }
}

impl Splode for Roid {
    fn splode(&self) -> ObjectSet {
        let new_radius = self.radius / 2.0;
        let num_sub_roids = if new_radius >= MIN_RADIUS { 2 } else { 0 };
        let roids = (0..num_sub_roids).map(|_| {
                let velocity = make_velocity_vector(self.speed() * 2.0, random_bearing());
                Roid::new(self.position, new_radius, velocity)
            })
            .collect();

        // TODO: Fragments
        //     for frag in explode(&self.position) {
        //         result.push((Category::Other, Box::new(frag)));
        //     }

        ObjectSet::from_objects(roids, vec![])
    }
}
