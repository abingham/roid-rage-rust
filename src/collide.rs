use crate::velocity::Velocity;
use nalgebra::geometry::Isometry2;
use nalgebra::{Point2, Vector2};
use ncollide2d::query;
use ncollide2d::shape::Shape;
use std::cmp::Ordering;
use std::iter::Iterator;

/// Things that can be collided together.
pub trait Collidable {
    fn collision_shape(&self) -> &dyn Shape<f64>;
    fn position(&self) -> &Point2<f64>;
    fn velocity(&self) -> &Vector2<f64>;
}

/// Collide two groups of Collidables together.
///
/// The return value is a sequence of Collidable pairs, each representing a collision between an object from the first
/// group and the second group. The first element of the pair is the object in `group1` and the second is the object in
/// `group2`.
pub fn collide<'a, T1, T2>(group1: &'a [T1], group2: &'a [T2], dt: f64) -> Vec<(&'a T1, &'a T2)>
where
    T1: Collidable,
    T2: Collidable,
{
    group1
        .iter()
        .map(|obj1| {
            let shape1 = obj1.collision_shape();
            let pos1 = Isometry2::new(
                Vector2::new(obj1.position().coords[0], obj1.position().coords[1]),
                0.0,
            );
            let pairs: Vec<(&'a T1, &'a T2)> = group2
                .iter()
                .filter_map(|obj2| {
                    let toi = query::time_of_impact(
                        &pos1,
                        obj1.velocity(),
                        shape1,
                        &Isometry2::new(
                            Vector2::new(obj2.position().coords[0], obj2.position().coords[1]),
                            0.0,
                        ),
                        obj2.velocity(),
                        obj2.collision_shape(),
                    );
                    match toi {
                        Some(t) => {
                            if t <= dt {
                                Some((obj1, obj2))
                            } else {
                                None
                            }
                        }
                        None => None,
                    }
                })
                .collect();
            pairs
        })
        .flatten()
        .collect()
}

/// Find real roots for a quadratic of the form:
///
/// ax^2 + bx + c
///
/// :param a: The "a" in the quadratic
/// :param b: The "b" in the quadratic
/// :param c: The "c" in the quadratic
/// :return: A list of real roots, sized 0, 1, or 2
fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    let disc = b.powf(2.0) - 4.0 * a * c;

    if disc < 0.0 {
        vec![]
    } else if disc == 0.0 {
        vec![(-1.0 * b) / (2.0 * a)]
    } else {
        let sqrt_disc = disc.sqrt();
        vec![
            (-1.0 * b + sqrt_disc) / (2.0 * a),
            (-1.0 * b - sqrt_disc) / (2.0 * a),
        ]
    }
}

pub fn collision_point(
    position: &Point2<f64>,
    speed: f64,
    target: &dyn Collidable
) -> Option<Point2<f64>> {
    let delta_x = position[0] - target.position()[0];
    let delta_y = position[1] - target.position()[1];

    let target_speed = target.velocity().speed();
    let target_bearing = target.velocity().bearing();
    let a = target_speed.powf(2.0) * f64::cos(target_bearing).powf(2.0) + target_speed.powf(2.0) * f64::sin(target_bearing).powf(2.0) - speed.powf(2.0);
    let b = -1.0 * (2.0 * delta_x * target_speed * f64::cos(target_bearing) + 2.0 * delta_y * target_speed * f64::sin(target_bearing));
    let c = delta_x.powf(2.0) + delta_y.powf(2.0);

    println!("target_speed {} target_bearing {} a {} b {} c {}", target_speed, target_bearing, a, b, c);

    solve_quadratic(a, b, c)
        .iter()
        .filter(|r| {
            **r >= 0.0
        })
        .min_by(|a, b| {
            a.partial_cmp(b).unwrap_or(Ordering::Less)
        })
        .map(|dt| {
            println!("dt: {}", dt);
            let coll_x = dt * target_speed * f64::cos(target_bearing) + target.position().x;
            let coll_y = dt * target_speed * f64::sin(target_bearing) + target.position().y;
            Point2::new(coll_x, coll_y)
        })
}

pub fn _collision_point(
    position: &Point2<f64>,
    speed: f64,
    target: &dyn Collidable,
) -> Option<Point2<f64>> {
    let dx = target.position().x - position.x;
    let dy = target.position().y - position.y;
    let target_speed = target.velocity().speed();
    let target_dir = target.velocity().bearing();
    let a = speed.powf(2.0) - target_speed.powf(2.0);
    let b = -2.0
        * (target_speed * f64::cos(target_dir) * dx + target_speed * f64::sin(target_dir) * dy);
    let c = -1.0 * dx.powf(2.0) + dy.powf(2.0);

    solve_quadratic(a, b, c)
        .iter()
        .filter(|r| {
            **r >= 0.0
        })
        .min_by(|a, b| {
            a.partial_cmp(b).unwrap_or(Ordering::Less)
        })
        .map(|dt| {
            let coll_x = dt * target_speed * f64::cos(target_dir) + target.position().x;
            let coll_y = dt * target_speed * f64::sin(target_dir) + target.position().y;
            Point2::new(coll_x, coll_y)
        })
}

/// Calculate vector from `position` to target that will result in
/// a collision given the other parameters.
///
/// :param position: The firing position.
/// :param speed: The speed of the projectile.
/// :param target_pos: Initial position of target to hit.
/// :param target_speed: Speed of target to hit.
/// :param target_dir: Direction (of movement) of target
///
/// :return: A tuple (collision-position, collision-vector) if one
pub fn collision_vector(
    position: &Point2<f64>,
    speed: f64,
    target: &dyn Collidable,
) -> Option<(Point2<f64>, Vector2<f64>)> {
    collision_point(position, speed, target).map(|p| (p, p - position))
}
