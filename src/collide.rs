// use crate::velocity::Velocity;
// use nalgebra::geometry::Isometry2;
// use nalgebra::{Point2, Vector2};
// use ncollide2d::query;
// use ncollide2d::shape::ShapeHandle;
// use std::cmp::Ordering;
// use std::iter::Iterator;

// /// Things that can be collided together.
// pub trait Collidable {
//     fn collision_shape(&self) -> ShapeHandle<f64>;
//     fn position(&self) -> &Point2<f64>;
//     fn velocity(&self) -> &Vector2<f64>;
// }

// /// Collide two groups of Collidables together.
// ///
// /// The return value is a sequence of Collidable pairs, each representing a collision between an object from the first
// /// group and the second group. The first element of the pair is the object in `group1` and the second is the object in
// /// `group2`.
// pub fn collide<'a, T1, T2>(group1: &'a [T1], group2: &'a [T2], dt: f64) -> Vec<(&'a T1, &'a T2)>
// where
//     T1: Collidable,
//     T2: Collidable,
// {
//     group1
//         .iter()
//         .map(|obj1| {
//             let shape1 = obj1.collision_shape();
//             let pos1 = Isometry2::new(
//                 Vector2::new(obj1.position().coords[0], obj1.position().coords[1]),
//                 0.0,
//             );
//             let pairs: Vec<(&'a T1, &'a T2)> = group2
//                 .iter()
//                 .filter_map(|obj2| {
//                     let toi = query::time_of_impact(
//                         &pos1,
//                         obj1.velocity(),
//                         shape1,
//                         &Isometry2::new(
//                             Vector2::new(obj2.position().coords[0], obj2.position().coords[1]),
//                             0.0,
//                         ),
//                         obj2.velocity(),
//                         obj2.collision_shape(),
//                     );
//                     match toi {
//                         Some(t) => {
//                             if t <= dt {
//                                 Some((obj1, obj2))
//                             } else {
//                                 None
//                             }
//                         }
//                         None => None,
//                     }
//                 })
//                 .collect();
//             pairs
//         })
//         .flatten()
//         .collect()
// }

// /// Find real roots for a quadratic of the form:
// ///
// /// ax^2 + bx + c
// ///
// /// :param a: The "a" in the quadratic
// /// :param b: The "b" in the quadratic
// /// :param c: The "c" in the quadratic
// /// :return: A list of real roots, sized 0, 1, or 2
// fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
//     let disc = b.powf(2.0) - 4.0 * a * c;

//     if disc < 0.0 {
//         vec![]
//     } else if disc == 0.0 {
//         vec![(-1.0 * b) / (2.0 * a)]
//     } else {
//         let sqrt_disc = disc.sqrt();
//         vec![
//             (-1.0 * b + sqrt_disc) / (2.0 * a),
//             (-1.0 * b - sqrt_disc) / (2.0 * a),
//         ]
//     }
// }

// /// Calculate the point at which a projectile would collide with a target.Collidable
// ///
// /// * `position` - where the projectile will be launched from
// /// * `speed` - the speed of the projectile
// /// * `target` - the thing we're looking to hit.
// ///
// /// The basic calculation is to find a time in the future when the distance of the projectile and the target is the
// /// same. This results in a quadratic equation which we solve. If this gives results, we choose the closest time, figure
// /// out where the target will be at that time, and return that.
// pub fn collision_point(
//     position: &Point2<f64>,
//     speed: f64,
//     target: &dyn Collidable,
// ) -> Option<Point2<f64>> {
//     let delta_x = position[0] - target.position()[0];
//     let delta_y = position[1] - target.position()[1];

//     let target_speed = target.velocity().speed();
//     let target_bearing = target.velocity().bearing();
//     let cos_target_bearing = f64::cos(target_bearing);
//     let sin_target_bearing = f64::sin(target_bearing);
//     let a = target_speed.powf(2.0) * cos_target_bearing.powf(2.0)
//         + target_speed.powf(2.0) * sin_target_bearing.powf(2.0)
//         - speed.powf(2.0);
//     let b = -1.0
//         * (2.0 * delta_x * target_speed * cos_target_bearing
//             + 2.0 * delta_y * target_speed * sin_target_bearing);
//     let c = delta_x.powf(2.0) + delta_y.powf(2.0);

//     solve_quadratic(a, b, c)
//         .iter()
//         .filter(|r| **r >= 0.0)
//         .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
//         .map(|dt| {
//             let coll_x = dt * target_speed * f64::cos(target_bearing) + target.position().x;
//             let coll_y = dt * target_speed * f64::sin(target_bearing) + target.position().y;
//             Point2::new(coll_x, coll_y)
//         })
// }

// /// Calculate vector from `position` to target that will result in
// /// a collision given the other parameters.
// ///
// /// * position - The firing position.
// /// * speed - The speed of the projectile.
// /// * target_pos - Initial position of target to hit.
// /// * target_speed - Speed of target to hit.
// /// * target_dir - Direction (of movement) of target
// ///
// /// Returns a tuple (collision-position, collision-vector) if one.
// pub fn collision_vector(
//     position: &Point2<f64>,
//     speed: f64,
//     target: &dyn Collidable,
// ) -> Option<(Point2<f64>, Vector2<f64>)> {
//     collision_point(position, speed, target).map(|p| (p, p - position))
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     mod quadratic {
//         use super::*;

//         #[test]
//         fn test_canned() {
//             let cases: Vec<((f64, f64, f64), Vec<f64>)> = vec![
//                 ((1.0, 9.0, 14.0), vec![-2.0, -7.0]),
//                 ((1.0, -9.0, 14.0), vec![7.0, 2.0]),
//                 ((1.0, 2.0, -15.0), vec![3.0, -5.0]),
//                 ((1.0, -2.0, -15.0), vec![5.0, -3.0]),
//                 ((2.0, 15.0, 27.0), vec![-3.0, -9.0 / 2.0]),
//                 ((2.0, -15.0, 27.0), vec![9.0 / 2.0, 3.0]),
//                 ((2.0, 3.0, -27.0), vec![3.0, -9.0 / 2.0]),
//             ];

//             for ((a, b, c), expected) in cases {
//                 assert_eq!(solve_quadratic(a, b, c), expected);
//             }
//         }
//     }
// }
