use crate::objects::GameObject;
use nalgebra::geometry::Isometry2;
use nalgebra::Vector2;
use ncollide2d::query;
use std::collections::HashSet;
use uuid::Uuid;

/// Collide two groups of GameObjects together.
///
/// The return value is a set of Uuid pairs, each representing a collision
/// between an object from the first group and the second group. The first
/// element of the pair is the ID of the object in `group1` and the secodn i the
/// ID of the object in `group2`.
pub fn collide<I: GameObject>(group1: &[I], group2: &[I], dt: f64) -> HashSet<(Uuid, Uuid)> {
    group1
        .iter()
        .map(|roid| {
            let shape1 = roid.collision_shape();
            let pos1 = Isometry2::new(
                Vector2::new(roid.position().coords[0], roid.position().coords[1]),
                0.0,
            );
            let colls: Vec<(Uuid, Uuid)> = group2
                .iter()
                .filter_map(|bullet| {
                    let shape2 = bullet.collision_shape();
                    let pos2 = Isometry2::new(
                        Vector2::new(bullet.position().coords[0], bullet.position().coords[1]),
                        0.0,
                    );
                    let toi = query::time_of_impact(
                        &pos1,
                        roid.velocity(),
                        shape1,
                        &pos2,
                        bullet.velocity(),
                        shape2,
                    );
                    match toi {
                        Some(t) => {
                            if t <= dt {
                                Some((roid.id(), bullet.id()))
                            } else {
                                None
                            }
                        }
                        None => None,
                    }
                })
                .collect();
            colls
        })
        .flatten()
        .collect()
}
