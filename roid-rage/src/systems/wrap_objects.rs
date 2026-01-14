use crate::components::{Position, Wrapping};
use crate::core::field::Field;
use glam::Vec2;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct WrapObjectsSystem;

/// Wrap entities that are supposed to wrap
impl<'s> System<'s> for WrapObjectsSystem {
    type SystemData = (
        WriteStorage<'s, Position>,
        ReadStorage<'s, Wrapping>,
        ReadExpect<'s, Field<f32>>,
    );

    fn run(&mut self, (mut positions, wrapping, field): Self::SystemData) {
        for (position, _wrapping) in (&mut positions, &wrapping).join() {
            let (x, y) = field.wrap(position.0.x, position.0.y);

            position.0 = Vec2::new(x, y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Position, Wrapping};
    use crate::core::field::Field;
    use specs::{Builder, World, WorldExt};

    #[test]
    fn wraps_only_wrapping_entities() {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Wrapping>();
        world.insert(Field::new(10.0_f32, 10.0_f32));

        world
            .create_entity()
            .with(Position(glam::Vec2::new(11.0, 5.0)))
            .with(Wrapping)
            .build();
        world
            .create_entity()
            .with(Position(glam::Vec2::new(11.0, 5.0)))
            .build();

        let mut system = WrapObjectsSystem;
        system.run_now(&world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        let wrapping = world.read_storage::<Wrapping>();
        for (position, _wrap) in (&positions, &wrapping).join() {
            assert_eq!(position.0, glam::Vec2::new(0.0, 5.0));
        }
        for (position, _entity) in (&positions, !&wrapping).join() {
            assert_eq!(position.0, glam::Vec2::new(11.0, 5.0));
        }
    }
}
