use crate::collide::collision_vector;
use crate::field::Field;
use crate::object_set::ObjectSet;
use crate::velocity::Velocity;
use nalgebra::Point2;
use std::cmp::Ordering;

/// Return the bearing of the shot to make, if any.
pub fn target(
    firing_position: &Point2<f64>,
    bullet_speed: f64,
    field: &Field,
    objects: &ObjectSet,
) -> Option<f64> {
    // Find the closest collision that will occurr in the field
    objects
        .roids()
        .filter_map(|roid| collision_vector(firing_position, bullet_speed, roid))
        .filter(|(p, _v)| field.contains(p))
        .min_by(|(p1, _v1), (p2, _v2)| {
            let d1 = (firing_position - p1).magnitude();
            let d2 = (firing_position - p2).magnitude();
            match d1.partial_cmp(&d2) {
                Some(ordering) => ordering,
                None => Ordering::Equal,
            }
        })
        .map(|(_p, v)| v.bearing())
}
