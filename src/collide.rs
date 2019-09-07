use nalgebra::{Point2, Vector2};
use nalgebra::geometry::Isometry2;
use ncollide2d::query;
use ncollide2d::shape::Shape;

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
