use crate::components::{Bullet, Transform};
use specs::{Join, ReadStorage, System};

pub struct LoggingSystem;

/// Wrap entities that are supposed to wrap
impl<'s> System<'s> for LoggingSystem {
    type SystemData = (ReadStorage<'s, Bullet>, ReadStorage<'s, Transform>);

    fn run(&mut self, (bullets, transforms): Self::SystemData) {
        for (_, transform) in (&bullets, &transforms).join() {
            println!(
                "x={} y={}",
                transform.0.translation.x,
                transform.0.translation.y
            );
        }
    }
}
