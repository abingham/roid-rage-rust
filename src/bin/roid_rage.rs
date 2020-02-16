extern crate roid_rage;

use nalgebra::Point2;
use rand::prelude::*;
use roid_rage::controller::basic_controller::BasicController;
use roid_rage::model::field::Field;
use roid_rage::model::object_set::ObjectSet;
use roid_rage::model::Model;
use roid_rage::model::objects::roid::Roid;
use roid_rage::velocity::{make_velocity_vector, random_bearing};
use roid_rage::view::View;

fn some_roids(width: usize, height: usize) -> Vec<Roid> {
    let mut rng = thread_rng();
    (1..10)
        .map(|_| {
            Roid::new(
                Point2::new(
                    rng.gen_range(0, width) as f64,
                    rng.gen_range(0, height) as f64,
                ),
                40.0,
                make_velocity_vector(100.0, random_bearing()),
            )
        })
        .collect()
}

// fn the_ship(width: usize, height: usize) -> (Category, Box<dyn GameObject>) {
//     (Category::Ship,
//      Box::new(
//         Ship::new(
//             Point2::new((width / 2) as f64, (height / 2) as f64),
//             make_velocity_vector(0.0, 0.0),
//             0.0)))
// }

fn main() {
    let mut objects = ObjectSet::new();
    objects.roids.extend(some_roids(800, 600));
    let model = Model::new(
        Field::new(800, 600, 100),
        objects
    );

    let controller = BasicController::new(model);

    let mut view = View::new(Box::new(controller), [800, 600]);
    view.run();
}