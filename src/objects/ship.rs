use nalgebra::{Point2, Vector2};
use ncollide2d::shape::{Polyline};
use uuid::Uuid;


pub struct Ship {
    collision_shape: Polyline<f64>,
    position: Point2<f64>,
    velocity: Vector2<f64>,
    heading: f64,
    id: Uuid,
    alive: bool,
}

impl Ship {
    pub fn new(position: Point2<f64>, velocity: Vector2<f64>, heading: f64) -> Ship {
        let points = vec![
            Point2::new(0.0, Ship::length() / 2.0),
            Point2::new(-1.0 * Ship::width() / 2.0, -1.0 * Ship::length() / 2.0),
            Point2::new(Ship::width() / 2.0, -1.0 * Ship::length() / 2.0),
            Point2::new(0.0, Ship::length() / 2.0) // This forms a loop.
        ];

        // Build the polyline.
        let polyline = Polyline::new(points, None);

        // assert!(polyline.vertices().len() == 4);

        Ship {
            position: position,
            velocity: velocity,
            heading: heading,
            id: Uuid::new_v4(),
            alive: true,
            collision_shape: polyline
        }
    }

    fn length() -> f64 {
        10.0
    }

    fn width() -> f64 {
        5.0
    }
}

// impl GameObject for Ship {
//     fn render(&self, color: &[f32; 4], c: graphics::Context, gl: &mut GlGraphics) {
//         use graphics::*;


//         let transform = c
//             .transform
//             .trans(self.position.coords[0], self.position.coords[1])
//             .rot_rad(self.heading); 

//         // TODO: How to create a triangle?
//         let body = rectangle::rectangle_by_corners(
//             -1.0 * Ship::width() / 2.0, 
//             -1.0 * Ship::length() / 2.0,
//             Ship::width() / 2.0,
//             Ship::length() / 2.0);

//         rectangle(*color, body, transform, gl);
//     }
    
//     fn update(&mut self, time_delta: f64) {
//         // TODO: How to use default implementation and augment it? Here we're duplicating code.
//         self.set_position(self.position() + self.velocity() * time_delta);

//         self.heading += 0.01;
//     }

//     fn collision_shape(&self) -> &dyn Shape<f64> {
//         &self.collision_shape
//     }

//     fn position(&self) -> &Point2<f64> {
//         &self.position
//     }

//     fn set_position(&mut self, pos: Point2<f64>) {
//         self.position = pos;
//     }

//     fn velocity(&self) -> &Vector2<f64> {
//         &self.velocity
//     }

//     fn id(&self) -> Uuid {
//         self.id
//     }

//     fn alive(&self) -> bool {
//         self.alive
//     }

//     fn kill(&mut self) -> Vec<(Category, Box<dyn GameObject>)> {
//         self.alive = false;
//         vec![]
//     }
// }
