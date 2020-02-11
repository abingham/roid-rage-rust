use crate::collide::collision_vector;
use crate::field::Field;
use crate::velocity::Velocity;
use nalgebra::{Point2, Vector2};
use std::cmp::Ordering;
use crate::game_object::{GameObject, Kind};

/// Return the bearing of the shot to make, if any.
pub fn target<'a, I>(
    firing_position: &Point2<f64>,
    bullet_speed: f64,
    field: &Field,
    objects: I,
) -> Option<f64>
where I: Iterator<Item = &'a dyn GameObject>
{
    // Find all possible collisions
    let hits: Vec<(Point2<f64>, Vector2<f64>)> = objects
        .filter(|obj| obj.kind() == Kind::Roid)
        .filter_map(|roid| collision_vector(firing_position, bullet_speed, *roid.position(), roid.velocity()))
        .filter(|(p, _v)| field.contains(p))
        .collect() ;

    // Return the bearing to the furthest collision, if any
    closest(firing_position, &hits).map(|(_p, v)| v.bearing())
}

/// Find the furthest away possible hit in a group.
fn _furthest<'a>(
    firing_position: &Point2<f64>,
    collisions: &'a [(Point2<f64>, Vector2<f64>)],
) -> Option<&'a (Point2<f64>, Vector2<f64>)> {
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
    firing_position: &Point2<f64>,
    collisions: &'a [(Point2<f64>, Vector2<f64>)],
) -> Option<&'a (Point2<f64>, Vector2<f64>)> {
    collisions.iter().min_by(|(p1, _v1), (p2, _v2)| {
        let d1 = (firing_position - p1).magnitude();
        let d2 = (firing_position - p2).magnitude();
        match d1.partial_cmp(&d2) {
            Some(ordering) => ordering,
            None => Ordering::Equal,
        }
    })
}
