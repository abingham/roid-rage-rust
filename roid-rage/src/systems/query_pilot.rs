use crate::components::{
    make_bullet, AngularVelocity, Bullet, LinearVelocity, Roid, Ship, TimeDelta,Position, Rotation
};
use crate::core::field::Field;
use crate::core::pilot;
use crate::settings::Settings;
use nalgebra::Point2;
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
        ReadStorage<'s, LinearVelocity>,
        WriteStorage<'s, AngularVelocity>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Rotation>,
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
            linear_velocities,
            mut angular_velocities,
            positions,
            rotations,
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

        for (ship, position, rotation, angular_velocity) in
            (&ships, &positions, &rotations, &mut angular_velocities).join()
        {
            let ship_center = position.0;
            // .x,
            //     transform.0.translation.vector.y,
            // );

            let firing_position = Point2::<f32>::new(
                ship_center.x + rotation.0.cos() * ship.length / 2.0,
                ship_center.y + rotation.0.sin() * ship.length / 2.0,
            );

            let game_state = pilot::GameState {
                field: field.clone(),
                firing_position: firing_position.clone(),
                firing_bearing: rotation.0,
                bullet_speed: settings.bullet_speed,
                time_to_fire: settings.rate_of_fire - self.fire_timer,

                roids: roids.clone(),
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
                            Point2::<f32>::new(firing_position.x, firing_position.y),
                            settings.bullet_speed,
                            rotation.0,
                            &mut collision_world,
                        );

                        // TODO: Angular velocity of ship should be in settings.
                        angular_velocity.0 = (command.rotation as f32) * 1.0;
                    }
                }
            }
        }
    }
}
