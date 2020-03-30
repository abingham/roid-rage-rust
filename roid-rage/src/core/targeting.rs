use crate::core::field::Field;
use nalgebra::{Point2, Vector2};
use std::cmp::Ordering;
// use crate::controller::velocity_model::VelocityModel;
use crate::core::collide::collision_vector;
use crate::core::velocity::Velocity;

/// Return the bearing of the shot to make, if any.
pub fn find_target<'a, I>(
    firing_position: &Point2<f32>,
    bullet_speed: f32,
    field: &Field,
    objects: I,
) -> Option<f32>
where
    I: Iterator<Item = (Point2<f32>, Vector2<f32>)>,
{
    // Find all possible collisions
    let hits: Vec<(Point2<f32>, Vector2<f32>)> = objects
        // .filter_map(|(id, position)| vmodel.velocity(id).map(|v| (position, v)))
        .filter_map(|(pos, vel)| collision_vector(firing_position, bullet_speed, &pos, &vel))
        .filter(|(p, _v)| field.contains(p.x, p.y))
        .collect();

    // Return the bearing to the furthest collision, if any
    closest(firing_position, &hits).map(|(_p, v)| v.bearing())
}

/// Find the furthest away possible hit in a group.
fn _furthest<'a>(
    firing_position: &Point2<f32>,
    collisions: &'a [(Point2<f32>, Vector2<f32>)],
) -> Option<&'a (Point2<f32>, Vector2<f32>)> {
    collisions.iter().max_by(|(p1, _v1), (p2, _v2)| {
        let d1 = (firing_position - p1).magnitude();
        let d2 = (firing_position - p2).magnitude();
        match d1.partial_cmp(&d2) {
            Some(ordering) => ordering,
            None => Ordering::Equal,
        }
    })
}

/// Find the closest possible hit in a group.
fn closest<'a>(
    firing_position: &Point2<f32>,
    collisions: &'a [(Point2<f32>, Vector2<f32>)],
) -> Option<&'a (Point2<f32>, Vector2<f32>)> {
    collisions.iter().min_by(|(p1, _v1), (p2, _v2)| {
        let d1 = (firing_position - p1).magnitude();
        let d2 = (firing_position - p2).magnitude();
        match d1.partial_cmp(&d2) {
            Some(ordering) => ordering,
            None => Ordering::Equal,
        }
    })
}
