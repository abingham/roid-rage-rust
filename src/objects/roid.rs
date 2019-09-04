use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Ball, Shape};
use opengl_graphics::GlGraphics;
use uuid::Uuid;

use super::categories::Category;
use super::game_object::GameObject;
use crate::explode::explode;
use crate::util::{make_velocity_vector, random_bearing};
use crate::collide::Collidable;

const MIN_RADIUS: f64 = 10.0;

pub struct Roid {
    radius: f64,
    collision_shape: Ball<f64>,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    id: Uuid,
    alive: bool,
}

impl Roid {
    pub fn new(position: Point2<f64>, radius: f64, velocity: Vector2<f64>) -> Roid {
        Roid {
            position: position,
            velocity: velocity,
            id: Uuid::new_v4(),
            alive: true,
            radius: radius,
            collision_shape: Ball::new(radius),
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
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

impl GameObject for Roid {
    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);
    }

    fn collision_shape(&self) -> &dyn Shape<f64> {
        &self.collision_shape
    }

    fn position(&self) -> &Point2<f64> {
        &self.position
    }
    fn set_position(&mut self, pos: Point2<f64>) {
        self.position = pos;
    }
    fn velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn alive(&self) -> bool {
        self.alive
    }
    fn kill(&mut self) -> Vec<(Category, Box<dyn GameObject>)> {
        self.alive = false;

        let mut result: Vec<(Category, Box<dyn GameObject>)> = vec![];

        for frag in explode(&self.position) {
            result.push((Category::Other, Box::new(frag)));
        }

        let new_radius = self.radius / 2.0;
        if new_radius >= MIN_RADIUS {
            for _ in 0..2 {
                let velocity = make_velocity_vector(self.speed() * 2.0, random_bearing());

                result.push((
                    Category::Roid,
                    Box::new(Roid::new(self.position, new_radius, velocity)),
                ));
            }
        }

        result
    }
}
