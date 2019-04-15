extern crate uuid;

use graphics;
use nalgebra::{Point2, Vector2};
use opengl_graphics::GlGraphics;
use uuid::Uuid;
use ncollide2d::shape::{Ball, Shape};

pub trait GameObject {
    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics);
    fn collision_shape(&self) -> &Shape<f64>;

    fn update(&mut self, time_delta: f64) {
        self.set_position(self.position() + self.velocity() * time_delta);
    }

    fn position(&self) -> &Point2<f64>;
    fn set_position(&mut self, pos: Point2<f64>);
    fn velocity(&self) -> &Vector2<f64>;
    fn id(&self) -> Uuid;
    fn alive(&self) -> bool;
    fn kill(&mut self);
}

macro_rules! game_object_type {
    ($struct:ident {$( $field:ident:$type:ty ),* }) => {
        pub struct $struct {
            position: Point2<f64>,
            velocity: Vector2<f64>,
            id: Uuid,
            alive: bool,
            $(
                $field: $type,
            )*
        }
    };
}

macro_rules! game_object_impl {
    () => {
        fn position(&self) -> &Point2<f64> { &self.position }
        fn set_position(&mut self, pos: Point2<f64>) { self.position = pos; }
        fn velocity(&self) -> &Vector2<f64> { &self.velocity }
        fn id(&self) -> Uuid { self.id }
        fn alive(&self) -> bool { self.alive }
        fn kill(&mut self) { self.alive = false }
    }
}

game_object_type!(Circle {
    radius: f64,
    collision_shape: Ball<f64>
});

impl Circle {
    pub fn new(position: Point2<f64>, radius: f64, velocity: Vector2<f64>) -> Circle {
        Circle {
            position: position,
            velocity: velocity,
            id: Uuid::new_v4(),
            alive: true,
            radius: radius,
            collision_shape: Ball::new(radius),
        }
    }

    pub fn radius(&self) -> f64 { self.radius }

}

impl GameObject for Circle {
    game_object_impl!();

    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);

    }

    fn collision_shape(&self) -> &Shape<f64> {
        &self.collision_shape
    }   
}

game_object_type!(Fragment {
    age: f64,
    max_age: f64,
    radius: f64,
    collision_shape: Ball<f64>
});

impl Fragment {
    pub fn new(position: Point2<f64>, velocity: Vector2<f64>, max_age: f64) -> Fragment {
        Fragment {
            position: position,
            velocity: velocity,
            id: Uuid::new_v4(),
            alive: true,
            age: 0.0,
            max_age: max_age,
            radius: 2.0,
            collision_shape: Ball::new(2.0),
        }
    }
}

impl GameObject for Fragment {
    game_object_impl!();

    fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) -> () {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);
    }

    fn update(&mut self, time_delta: f64) {
        self.set_position(self.position() + self.velocity() * time_delta);
        self.age += time_delta;
        if self.age > self.max_age {
            self.kill();
        }
    }

    fn collision_shape(&self) -> &Shape<f64> { &self.collision_shape }
}