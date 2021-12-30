use crate::components::{make_ship, Ship};
use sted::Bearing;
use crate::settings::Settings;
use ncollide2d::world::CollisionWorld;
use specs::{Entities, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect};

/// Respawn the ship if needed
pub struct RespawnShipSystem;

impl<'s> System<'s> for RespawnShipSystem {
    type SystemData = (
        ReadStorage<'s, Ship>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, specs::world::Index>>,
        ReadExpect<'s, Settings>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (ships, entities, mut collision_world, settings, lazy): Self::SystemData) {
        if !ships.is_empty() {
            return;
        }

        let x = settings.screen_width / 2.0;
        let y = settings.screen_height / 2.0;
        let speed = 0.0;
        let course = Bearing::new(0.0);
        let heading = Bearing::new(0.0);
        let new_entity = entities.create();
        make_ship(
            specs::world::LazyBuilder {
                entity: new_entity,
                lazy: &*lazy,
            },
            heading,
            settings.ship_length,
            settings.ship_width,
            settings.ship_mass,
            settings.ship_thrust,
            settings.ship_rotational_speed,
            x,
            y,
            speed,
            course,
            &mut collision_world,
        );
    }
}
