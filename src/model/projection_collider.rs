use ncollide2d::pipeline::CollisionObjectSlabHandle;
use std::collections::HashMap;
use super::explodable::Explodable;
use super::field::Field;
use super::object_set::ObjectSet;
use super::traits::*;

/// Project and collide a group of objects.
pub trait ProjectionCollider {
    /// Process the managed elements, returning information about those objects which have been removed (e.g. because they
    /// fell off the field) and the debris produced by any collisions.
    fn project(&mut self, 
               time_delta: f64,
               collisions: &Vec<CollisionObjectSlabHandle>,
               field: &Field) -> (Vec<CollisionObjectSlabHandle>, ObjectSet);
}

impl<T: Explodable + Identifiable + Positioned> ProjectionCollider for HashMap<CollisionObjectSlabHandle, T>
{
    fn project(&mut self, 
               time_delta: f64,
               collisions: &Vec<CollisionObjectSlabHandle>,
               field: &Field) -> (Vec<CollisionObjectSlabHandle>, ObjectSet)
    {
        // Collect the objects that have exploded, removing them from the objects.
        let (mut removals, exploded): (Vec<CollisionObjectSlabHandle>, Vec<T>) = collisions.iter()
            .filter_map(|handle| self.remove(handle).and_then(|o| Some((*handle, o))))
            .unzip();

        // Move everything    
        for (_handle, obj) in &mut self.iter_mut() {
            obj.project(field, time_delta);
        }

        // Trim out the objects that have moved outside the field or otherwise died
        // let mut removals: Vec<CollisionObjectSlabHandle> = 
        removals.extend(
            self.iter()
                .filter(|(_h, o)| !field.contains(&o.position()) || !o.alive())
                .map(|(h, _o)| *h));
        removals.dedup();
        removals.sort();
        self.retain(|handle, _obj| removals.binary_search(&handle).is_err());

       let debris = exploded
            .iter()
            .map(|x| x.explode())
            .fold(ObjectSet::new(), |mut acc, x| {
                acc.extend(x);
                acc
            });

        (removals, debris)
    }
}

