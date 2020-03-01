use crate::components::Transform;
use specs::{Join, ReadStorage, System};

pub struct LoggingSystem;

/// Wrap entities that are supposed to wrap
impl<'s> System<'s> for LoggingSystem {
    type SystemData = ReadStorage<'s, Transform>;

    fn run(&mut self, transforms: Self::SystemData) {
        for transform in transforms.join() {
            println!(
                "x={} y={}",
                transform.0.translation.x,
                transform.0.translation.y
            );
        }
    }
}
