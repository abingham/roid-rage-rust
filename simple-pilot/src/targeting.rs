use glam::Vec2;
// use roid_rage::core::collide::collision_vector;
use roid_rage::core::field::Field;
use std::cmp::Ordering;
// use sted::Velocity;

use roid_rage_grpc::roid_rage as rpc;

// type Point2 = Vec2;

/// Return the bearing of the shot to make, if any.
pub fn find_target(
    firing_position: &Vec2,
    bullet_speed: f32,
    field: &Field,
    objects: &[rpc::Roid],
) -> Option<f32> {
    None
    // // Find all possible collisions
    // let hits: Vec<(Vec2, Vec2)> = objects
    //     .iter()
    //     .filter_map(|r| collision_vector(firing_position, bullet_speed, &r.position, &r.velocity))
    //     .filter(|(p, _v)| field.contains(p.x, p.y))
    //     .collect();

    // // Return the bearing to the furthest collision, if any
    // closest(firing_position, &hits).map(|(_p, v)| v.bearing())
}

// /// Find the furthest away possible hit in a group.
// fn _furthest<'a>(
//     firing_position: &Point2,
//     collisions: &'a [(Point2, Vec2)],
// ) -> Option<&'a (Point2, Vec2)> {
//     collisions.iter().max_by(|(p1, _v1), (p2, _v2)| {
//         let d1 = (*firing_position - *p1).length();
//         let d2 = (*firing_position - *p2).length();
//         match d1.partial_cmp(&d2) {
//             Some(ordering) => ordering,
//             None => Ordering::Equal,
//         }
//     })
// }

/// Find the closest possible hit in a group.
fn closest<'a>(firing_position: &Vec2, collisions: &'a [(Vec2, Vec2)]) -> Option<&'a (Vec2, Vec2)> {
    collisions.iter().min_by(|(p1, _v1), (p2, _v2)| {
        let d1 = (*firing_position - *p1).length();
        let d2 = (*firing_position - *p2).length();
        match d1.partial_cmp(&d2) {
            Some(ordering) => ordering,
            None => Ordering::Equal,
        }
    })
}
