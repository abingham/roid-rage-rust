use amethyst::core::transform::Transform;
use amethyst::ecs::{Join, ReadExpect, ReadStorage, System};

pub struct LoggingSystem;

/// Wrap entities that are supposed to wrap
impl<'s> System<'s> for LoggingSystem {
    type SystemData = ReadStorage<'s, Transform>;

    fn run(&mut self, transforms: Self::SystemData) {
        for transform in transforms.join() {
            println!("x={} y={}", transform.translation().x, transform.translation().y);
        }
    }
}
