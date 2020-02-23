use crate::components::Wrapping;
use crate::field::Field;
use amethyst::core::transform::Transform;
use amethyst::ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct WrappingSystem;

/// Wrap entities that are supposed to wrap
impl<'s> System<'s> for WrappingSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Wrapping>,
        ReadExpect<'s, Field>,
    );

    fn run(&mut self, (mut transforms, wrapping, field): Self::SystemData) {
        for (transform, _wrapping) in (&mut transforms, &wrapping).join() {
            let (x, y) = field.wrap(transform.translation().x, transform.translation().y);
            transform.set_translation_x(x);
            transform.set_translation_y(y);
        }
    }
}
