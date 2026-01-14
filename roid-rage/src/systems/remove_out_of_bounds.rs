use crate::components::{Position, Wrapping};
use crate::core::field::Field;
use specs::{Entities, Join, ReadExpect, ReadStorage, System};

pub struct RemoveOutOfBoundsSystem;

/// Delete entities that have left the field
impl<'s> System<'s> for RemoveOutOfBoundsSystem {
    type SystemData = (
        ReadStorage<'s, Position>,
        ReadStorage<'s, Wrapping>,
        Entities<'s>,
        ReadExpect<'s, Field<f32>>,
    );

    fn run(&mut self, (positions, wrapping, entities, field): Self::SystemData) {
        for (position, _wrap, entity) in (&positions, !&wrapping, &entities).join() {
            if !field.contains(position.0.x, position.0.y) {
                match entities.delete(entity) {
                    Err(e) => println!("Error deleting entity: {}", e),
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Position, Wrapping};
    use crate::core::field::Field;
    use specs::{Builder, RunNow, World, WorldExt};

    #[test]
    fn removes_entities_outside_field() {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Wrapping>();
        world.insert(Field::new(10.0_f32, 10.0_f32));

        let outside = world
            .create_entity()
            .with(Position(glam::Vec2::new(11.0, 5.0)))
            .build();
        let inside = world
            .create_entity()
            .with(Position(glam::Vec2::new(1.0, 2.0)))
            .build();

        let mut system = RemoveOutOfBoundsSystem;
        system.run_now(&world);
        world.maintain();

        let entities = world.entities();
        assert!(!entities.is_alive(outside));
        assert!(entities.is_alive(inside));
    }

    #[test]
    fn ignores_wrapping_entities() {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Wrapping>();
        world.insert(Field::new(10.0_f32, 10.0_f32));

        let wrapped = world
            .create_entity()
            .with(Position(glam::Vec2::new(11.0, 5.0)))
            .with(Wrapping)
            .build();

        let mut system = RemoveOutOfBoundsSystem;
        system.run_now(&world);
        world.maintain();

        let entities = world.entities();
        assert!(entities.is_alive(wrapped));
    }
}
