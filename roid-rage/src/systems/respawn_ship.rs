/// System responsible for creating ships for pilots with no ships.
use crate::components::{make_ship, Cannon, Pilot, Ship};
use crate::settings::Settings;
use specs::{Builder, Entities, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect};
use sted::Bearing;

/// Respawn the ship if needed
pub struct RespawnShipSystem;

impl<'s> System<'s> for RespawnShipSystem {
    type SystemData = (
        ReadStorage<'s, Ship>,
        Entities<'s>,
        ReadExpect<'s, Settings>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (ships, entities, settings, lazy): Self::SystemData) {
        if !ships.is_empty() {
            return;
        }

        let position_x = settings.screen_width / 2.0;
        let position_y = settings.screen_height / 2.0;
        let speed = 0.0;
        let course = Bearing::new(0.0);
        let heading = Bearing::new(0.0);
        let new_entity = entities.create();

        // Arrange for this ship to be created.
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
            Cannon {
                bullet_speed: settings.bullet_speed,
                rate_of_fire: settings.rate_of_fire,
            },
            position_x,
            position_y,
            speed,
            course,
        )
        .with(Pilot::new())
        .build();
    }
}
