use nalgebra::{Point2, Vector2};
use nalgebra::geometry::Isometry2;
use ncollide2d::query;
use ncollide2d::shape::Shape;

use crate::util;

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
pub fn collide<'a, T1, T2>(
    group1: &'a [T1],
    group2: &'a [T2],
    dt: f64,
) -> Vec<(&'a T1, &'a T2)>
where
    T1: Collidable,
    T2: Collidable
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
                            0.0),
                        obj2.velocity(),
                        obj2.collision_shape());
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

pub fn collision_point(position: &Point2<f64>, speed: f64, target: &dyn Collidable) -> Option<Point2<f64>> {
    let dx = target.position().x - position.x;
    let dy = target.position().y - position.y;
    let target_speed = crate::util::speed(target.velocity())
    let a = speed.powf(2.0) - target_speed(target.velocity()).powf(2.0);
    let b = -2 * (target_speed * math.cos())
//     b = -2 * (target_speed * math.cos(target_dir) * dx +
//               target_speed * math.sin(target_dir) * dy)
//     c = -1 * (pow(dx, 2) + pow(dy, 2))

//     roots = [r for r in solve_quadratic(a, b, c) if r >= 0]
//     if not roots:
//         return None

//     # This is how far in the future the collision will occur
//     dt = min(roots) if roots else None

//     coll_x = dt * target_speed * math.cos(target_dir) + target_pos.x
//     coll_y = dt * target_speed * math.sin(target_dir) + target_pos.y
//     return geom.Point(coll_x, coll_y)

    None
}
// def calculate_collision_point(
//         position,
//         speed,  # speed of projectile
//         target_pos,
//         target_speed,  # speed of target
//         target_dir):
//     '''Calculate point of collision given target position + velocity
//     and projectile speed + firing position.

//     :return: Point of collision, or None if no collision possible
//     '''

//     # Solve for delta-t, the time at which the target and the
//     # projectile will be equally distant from `position`. This
//     # involves solving a quadratic equation, hence the a, b, and c.
//     dx = target_pos.x - position.x
//     dy = target_pos.y - position.y
//     a = pow(speed, 2) - pow(target_speed, 2)
//     b = -2 * (target_speed * math.cos(target_dir) * dx +
//               target_speed * math.sin(target_dir) * dy)
//     c = -1 * (pow(dx, 2) + pow(dy, 2))

//     roots = [r for r in solve_quadratic(a, b, c) if r >= 0]
//     if not roots:
//         return None

//     # This is how far in the future the collision will occur
//     dt = min(roots) if roots else None

//     coll_x = dt * target_speed * math.cos(target_dir) + target_pos.x
//     coll_y = dt * target_speed * math.sin(target_dir) + target_pos.y
//     return geom.Point(coll_x, coll_y)


// def calculate_collision_vector(position,
//                                speed,
//                                target_pos,
//                                target_speed,
//                                target_dir):
//     '''Calculate vector from `position` to target that will result in
//     a collision given the other parameters.

//     :param position: The firing position.
//     :param speed: The speed of the projectile.
//     :param target_pos: Initial position of target to hit.
//     :param target_speed: Speed of target to hit.
//     :param target_dir: Direction (of movement) of target

//     :return: A tuple (collision-position, collision-vector) if one
//         exists, or (None, None) if not.
//     '''

//     coll_pos = calculate_collision_point(
//         position=position,
//         speed=speed,
//         target_pos=target_pos,
//         target_speed=target_speed,
//         target_dir=target_dir)

//     if coll_pos is None:
//         return (None, None)

//     return (coll_pos, coll_pos - position)