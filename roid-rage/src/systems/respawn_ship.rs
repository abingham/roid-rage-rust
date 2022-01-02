/// System responsible for creating ships for pilots with no ships.
use crate::components::{make_ship, Cannon, Pilot, Ship};
use crate::settings::Settings;
use specs::{
    Builder, EntityBuilder, Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage
};
use sted::Bearing;

/// Respawn the ship if needed
pub struct RespawnShipSystem;

impl<'s> System<'s> for RespawnShipSystem {
    type SystemData = (
        WriteStorage<'s, Pilot>,
        ReadStorage<'s, Ship>,
        Entities<'s>,
        ReadExpect<'s, Settings>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (mut pilots, ships, entities, settings, lazy): Self::SystemData) {
        // TEMPORARY: If there are no pilots, create one without a ship.
        // TODO: Eventually this will happen as a result of a message from a new pilot process.
        if pilots.is_empty() {
            let new_entity = entities.create();
            match pilots.insert(new_entity, Pilot::new()) {
                Err(_) => println!("oops! Trouble creating pilot"),
                Ok(_) => println!("new pilot")
            }
        }
        
        // Find all pilots without a ship.
        for (_pilot, entity, ()) in (&pilots, &entities, !&ships).join() {
            let position_x = settings.screen_width / 2.0;
            let position_y = settings.screen_height / 2.0;
            let speed = 0.0;
            let course = Bearing::new(0.0);
            let heading = Bearing::new(0.0);

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
                speed,
                course,
            )
            .build();
        }
    }
}
