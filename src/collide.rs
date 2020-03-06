use crate::core::velocity::Velocity;
use nalgebra::*;
use std::cmp::Ordering;

/// Find real roots for a quadratic of the form:
///
/// ax^2 + bx + c
///
/// :param a: The "a" in the quadratic
/// :param b: The "b" in the quadratic
/// :param c: The "c" in the quadratic
/// :return: A list of real roots, sized 0, 1, or 2
fn solve_quadratic(a: f32, b: f32, c: f32) -> Vec<f32> {
    let disc: f32 = b.powf(2.0) - 4.0 * a * c;

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

/// Calculate the point at which a projectile would collide with a target.Collidable
///
/// * `position` - where the projectile will be launched from
/// * `speed` - the speed of the projectile
/// * `target` - the thing we're looking to hit.
///
/// The basic calculation is to find a time in the future when the distance of the projectile and the target is the
/// same. This results in a quadratic equation which we solve. If this gives results, we choose the closest time, figure
/// out where the target will be at that time, and return that.
pub fn collision_point(
    position: &Point2<f32>,
    speed: f32,
    target_position: Point2<f32>,
    target_velocity: Vector2<f32>,
) -> Option<Point2<f32>> 
{
    let delta_x = position[0] - target_position.x;
    let delta_y = position[1] - target_position.y;

    let cos_target_bearing = f32::cos(target_velocity.bearing());
    let sin_target_bearing = f32::sin(target_velocity.bearing());
    let a = target_velocity.speed().powf(2.0) * cos_target_bearing.powf(2.0)
        + target_velocity.speed().powf(2.0) * sin_target_bearing.powf(2.0)
        - speed.powf(2.0);
    let b = -1.0 
        * (2.0 * delta_x * target_velocity.speed() * cos_target_bearing
            + 2.0 * delta_y * target_velocity.speed() * sin_target_bearing);
    let c = delta_x.powf(2.0) + delta_y.powf(2.0);

    solve_quadratic(a, b, c)
        .iter()
        .filter(|r| **r >= 0.0)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
        .map(|dt| {
            let coll_x = *dt * target_velocity.speed() * f32::cos(target_velocity.bearing()) + target_position.x;
            let coll_y = *dt * target_velocity.speed() * f32::sin(target_velocity.bearing()) + target_position.y;
            Point2::new(coll_x, coll_y)
        })
}

/// Calculate vector from `position` to target that will result in
/// a collision given the other parameters.
///
/// * position - The firing position.
/// * speed - The speed of the projectile.
/// * target_pos - Initial position of target to hit.
/// * target_speed - Speed of target to hit.
/// * target_dir - Direction (of movement) of target
///
/// Returns a tuple (collision-position, collision-vector) if one.
pub fn collision_vector(
    position: &Point2<f32>,
    speed: f32,
    target_position: Point2<f32>,
    target_velocity: Vector2<f32>,
) -> Option<(Point2<f32>, Vector2<f32>)> 
{
    collision_point(position, speed, target_position, target_velocity)
        .map(|p| {
            (p, p - position)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod quadratic {
        use super::*;

        #[test]
        fn test_canned() {
            let cases: Vec<((f32, f32, f32), Vec<f32>)> = vec![
                ((1.0, 9.0, 14.0), vec![-2.0, -7.0]),
                ((1.0, -9.0, 14.0), vec![7.0, 2.0]),
                ((1.0, 2.0, -15.0), vec![3.0, -5.0]),
                ((1.0, -2.0, -15.0), vec![5.0, -3.0]),
                ((2.0, 15.0, 27.0), vec![-3.0, -9.0 / 2.0]),
                ((2.0, -15.0, 27.0), vec![9.0 / 2.0, 3.0]),
                ((2.0, 3.0, -27.0), vec![3.0, -9.0 / 2.0]),
            ];

            for ((a, b, c), expected) in cases {
                assert_eq!(solve_quadratic(a, b, c), expected);
            }
        }
    }
}
