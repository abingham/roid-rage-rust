use crate::components::{Transform, Wrapping};
use crate::core::field::Field;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct WrapObjectsSystem;

/// Wrap entities that are supposed to wrap
impl<'s> System<'s> for WrapObjectsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Wrapping>,
        ReadExpect<'s, Field>,
    );

    fn run(&mut self, (mut transforms, wrapping, field): Self::SystemData) {
        for (transform, _wrapping) in (&mut transforms, &wrapping).join() {
            let (x, y) = field.wrap(transform.0.translation.x, transform.0.translation.y);

            transform.0.translation.vector.x = x;
            transform.0.translation.vector.y = y;
        }
    }
}
