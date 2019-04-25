use crate::objects::fragment::Fragment;
use crate::util::make_velocity_vector;
use nalgebra::Point2;
use rand::prelude::*;
use std::f64::consts::PI;

pub fn explode(pos: &Point2<f64>) -> Vec<Fragment> {
    let mut rng = thread_rng();

    let num_fragments: i32 = rng.gen_range(1, 4);

    (0..num_fragments)
        .map(|_| {
            let bearing = rng.gen::<f64>();
            let speed = rng.gen::<f64>() * 400.0 + 200.0;
            let age = rng.gen::<f64>() * 0.5;
            Fragment::new(
                *pos,
                make_velocity_vector(speed, (bearing * 2.0 - 1.0) * PI),
                age,
            )
        })
        .collect()
}
