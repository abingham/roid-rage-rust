// use crate::objects::fragment::Fragment;
// use crate::velocity::{make_velocity_vector, random_bearing};
// use nalgebra::Point2;
// use rand::prelude::*;

// pub fn make_explosion(pos: &Point2<f64>) -> Vec<Fragment> {
//     let mut rng = thread_rng();

//     (0..rng.gen_range(1, 10))
//         .map(|_| {
//             let speed = rng.gen::<f64>() * 400.0 + 200.0;
//             let max_age = rng.gen::<f64>() * 0.5;
//             Fragment::new(
//                 *pos,
//                 make_velocity_vector(speed, random_bearing()),
//                 0.0,
//                 max_age,
//             )
//         })
//         .collect()
// }
