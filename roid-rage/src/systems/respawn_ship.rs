/// System responsible for creating ships for pilots with no ships.
use crate::components::{make_ship, Cannon, Pilot, Ship};
use crate::settings::Settings;
use specs::{Builder, Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System};

/// Respawn the ship if needed
pub struct RespawnShipSystem;

impl<'s> System<'s> for RespawnShipSystem {
    type SystemData = (
        ReadStorage<'s, Pilot>,
        ReadStorage<'s, Ship>,
        Entities<'s>,
        ReadExpect<'s, Settings>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (pilots, ships, entities, settings, lazy): Self::SystemData) {
        // Find all pilots without a ship.
        for (_pilot, entity, ()) in (&pilots, &entities, !&ships).join() {
            let position_x = settings.screen_width / 2.0;
            let position_y = settings.screen_height / 2.0;
            let heading = glam::Vec2::ZERO;

            // Arrange for a ship to be created.
            make_ship(
                specs::world::LazyBuilder {
                    entity: entity,
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
                glam::Vec2::ZERO,
            )
            .build();
        }
    }
}
