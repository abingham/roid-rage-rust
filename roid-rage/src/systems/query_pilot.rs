use crate::components::{
    make_bullet, Bullet, LinearVelocity, Position, Roid, Rotation, Ship, TimeDelta,
};
use crate::core::field::Field;
use crate::core::pilot;
use crate::core::util::from_quantity_and_bearing;
use crate::settings::Settings;
use glam::Vec2;
use ncollide2d::world::CollisionWorld;
use specs::{
    Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage,
};

pub struct QueryPilotSystem {
    fire_timer: f32,
}

impl QueryPilotSystem {
    pub fn new() -> QueryPilotSystem {
        QueryPilotSystem { fire_timer: 0.0 }
    }
}

/// Identify target and shoot a bullet
impl<'s> System<'s> for QueryPilotSystem {
    type SystemData = (
        ReadStorage<'s, Roid>,
        ReadStorage<'s, Ship>,
        WriteStorage<'s, LinearVelocity>,
        ReadStorage<'s, Position>,
        WriteStorage<'s, Rotation>,
        WriteStorage<'s, Bullet>,
        ReadExpect<'s, Field>,
        Read<'s, TimeDelta>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, specs::world::Index>>,
        ReadExpect<'s, Settings>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            roids,
            ships,
            mut linear_velocities,
            positions,
            mut rotations,
            _bullets,
            field,
            time_delta,
            entities,
            mut collision_world,
            settings,
            lazy,
        ): Self::SystemData,
    ) {
        self.fire_timer += time_delta.0.as_secs_f32();

        let roids: Vec<pilot::Roid> = (&roids, &linear_velocities, &positions, &entities)
            .join()
            .map(|(roid, linear_velocity, position, entity)| pilot::Roid {
                id: entity.id(),
                radius: roid.radius,
                position: position.0,
                velocity: linear_velocity.0,
            })
            .collect();

        for (ship, position, rotation, linear_velocity) in (
            &ships,
            &positions,
            &mut rotations,
            &mut linear_velocities,
        )
            .join()
        {
            let ship_center = position.0;

            let firing_position = Vec2::new(
                ship_center.x + rotation.0.radians().cos() * ship.length / 2.0,
                ship_center.y + rotation.0.radians().sin() * ship.length / 2.0,
            );

            let game_state = pilot::GameState {
                field: field.clone(),
                firing_position: firing_position.clone(),
                firing_velocity: from_quantity_and_bearing(
                    settings.bullet_speed,
                    rotation.0.radians()
                ),
                time_to_fire: settings.rate_of_fire - self.fire_timer,
                roids: roids.clone(),
                ship: pilot::Ship {
                    mass: ship.mass,
                    thrust: ship.thrust,
                    position: ship_center,
                    velocity: linear_velocity.0,
                    heading: rotation.0.radians(),
                },
            };

            // Pass game-state to pilot process
            let result = pilot::query_pilot(&settings.pilot_url, &game_state);

            match result {
                Err(msg) => println!("Error communicating with pilot: {:?}", msg),
                Ok(command) => {
                    if command.fire && self.fire_timer >= settings.rate_of_fire {
                        self.fire_timer = 0.0;

                        let new_entity = entities.create();
                        make_bullet(
                            specs::world::LazyBuilder {
                                entity: new_entity,
                                lazy: &*lazy,
                            },
                            firing_position,
                            settings.bullet_speed,
                            rotation.0.radians(),
                            &mut collision_world,
                        );
                    }

                    let rotation_direction = match command.rotation {
                        pilot::Rotation::Clockwise => 1.0,
                        pilot::Rotation::Counterclockwise => -1.0,
                        pilot::Rotation::None => 0.0,
                    };

                    rotation.0 = rotation.0 + rotation_direction * ship.rotational_speed * time_delta.0.as_secs_f32();

                    if command.thrusters {
                        let steering_force = from_quantity_and_bearing(ship.thrust, rotation.0.radians());
                        let accel = steering_force / ship.mass;
                        linear_velocity.0 += accel * time_delta.0.as_secs_f32();
                    }
                }
            }
        }
    }
}
