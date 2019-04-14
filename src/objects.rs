use opengl_graphics::GlGraphics;
use graphics;
use nalgebra::Point2;
use nalgebra::geometry::Rotation2;
use nalgebra::Vector2;

pub struct Circle {
    pub position: Point2<f64>,
    pub radius: f64,
    pub speed: f64,
    pub bearing: f64,
}

impl Circle {
    pub fn render(&self, color: &[f32;4], c: graphics::Context, gl: &mut GlGraphics) -> () {
        use graphics::*;

        let transform = c
            .transform
            .trans(self.position.coords[0], self.position.coords[1]);

        let rect = rectangle::square(-1.0 * self.radius, -1.0 * self.radius, 2.0 * self.radius);
        ellipse(*color, rect, transform, gl);
    }

    pub fn update(&mut self, time_delta: f64) -> () {
        self.position = move_point(&self.position, self.speed * time_delta, self.bearing);    
    }
}

fn move_point(point: &Point2<f64>, distance: f64, bearing: f64) -> Point2<f64> {
    let rot = Rotation2::new(bearing);
    let vec = rot.transform_vector(&Vector2::new(distance, 0.0));
    point + vec
}

