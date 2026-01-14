use nalgebra::Vector2;
use num::{Float, FromPrimitive};
use std::cmp::Ordering;

fn speed<T: Float>(vector: &Vector2<T>) -> T {
    (vector[0] * vector[0] + vector[1] * vector[1]).sqrt()
}

fn bearing<T: Float>(vector: &Vector2<T>) -> T {
    vector[1].atan2(vector[0])
}

/// Find real roots for a quadratic of the form:
///
/// ax^2 + bx + c
///
/// :param a: The "a" in the quadratic
/// :param b: The "b" in the quadratic
/// :param c: The "c" in the quadratic
/// :return: A list of real roots, sized 0, 1, or 2
fn solve_quadratic<T>(a: T, b: T, c: T) -> Vec<T>
where
    T: Float + FromPrimitive,
{
    let four = T::from_u8(4).unwrap();
    let disc: T = b.powi(2) - four * a * c;

    if disc < T::zero() {
        vec![]
    } else if disc == T::zero() {
        let two = T::from_u8(2).unwrap();
        vec![(-T::one() * b) / (two * a)]
    } else {
        let sqrt_disc = disc.sqrt();
        let two = T::from_u8(2).unwrap();
        vec![
            (-T::one() * b + sqrt_disc) / (two * a),
            (-T::one() * b - sqrt_disc) / (two * a),
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
pub fn collision_point<T>(
    launch_position: &Vector2<T>,
    projectile_speed: T,
    target_position: &Vector2<T>,
    target_velocity: &Vector2<T>,
) -> Option<Vector2<T>>
where
    T: Float + FromPrimitive,
{
    let delta_x = launch_position[0] - target_position[0];
    let delta_y = launch_position[1] - target_position[1];

    let target_speed = speed(target_velocity);
    let target_bearing = bearing(target_velocity);
    let cos_target_bearing = target_bearing.cos();
    let sin_target_bearing = target_bearing.sin();
    let two = T::from_u8(2).unwrap();
    let a = target_speed.powi(2) * cos_target_bearing.powi(2)
        + target_speed.powi(2) * sin_target_bearing.powi(2)
        - projectile_speed.powi(2);
    let b = -T::one()
        * (two * delta_x * target_speed * cos_target_bearing
            + two * delta_y * target_speed * sin_target_bearing);
    let c = delta_x.powi(2) + delta_y.powi(2);

    solve_quadratic(a, b, c)
        .iter()
        .filter(|r| **r >= T::zero())
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
        .map(|dt| {
            let coll_x = *dt * target_speed * target_bearing.cos() + target_position[0];
            let coll_y = *dt * target_speed * target_bearing.sin() + target_position[1];
            Vector2::new(coll_x, coll_y)
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
pub fn collision_vector<T>(
    position: &Vector2<T>,
    speed: T,
    target_position: &Vector2<T>,
    target_velocity: &Vector2<T>,
) -> Option<(Vector2<T>, Vector2<T>)>
where
    T: Float + FromPrimitive,
{
    collision_point(position, speed, target_position, target_velocity)
        .map(|p| {
            let vector = Vector2::new(p[0] - position[0], p[1] - position[1]);
            (p, vector)
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
