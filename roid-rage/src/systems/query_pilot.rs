use crate::components::{make_bullet, Bullet, LinearVelocity, Roid, TimeDelta, Transform};
use crate::core::field::Field;
use crate::pilot;
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
        QueryPilotSystem {
            fire_timer: 0.0,
        }
    }
}

/// Identify target and shoot a bullet
impl<'s> System<'s> for QueryPilotSystem {
    type SystemData = (
        ReadStorage<'s, Roid>,
        ReadStorage<'s, LinearVelocity>,
        ReadStorage<'s, Transform>,
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
            linear_velocities,
            transforms,
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
        // if self.fire_timer <= settings.rate_of_fire {
        //     return;
        // }

        let roids = (&roids, &linear_velocities, &transforms, &entities).join().map(
            |(roid, linear_velocity, transform, entity)| {
                crate::pilot::Roid {
                    id: entity.id(),
                    radius: roid.radius,
                    position: pilot::Point::from(transform.0.translation.vector),
                    velocity: pilot::Point::from(linear_velocity.0),
                }
            },
        );
        let firing_position = Point2::<f32>::new(settings.screen_width / 2.0, settings.screen_height / 2.0);

        let game_state = crate::pilot::GameState {
            field: field.clone(),    
            firing_position: pilot::Point::from(firing_position),
            bullet_speed: settings.bullet_speed,
            roids: roids.collect(),
        };

        // Pass game-state to pilot process
        // TODO: Get host from settings.
        let result = pilot::query_pilot("localhost:8000", &game_state);

        match result {
            Err(msg) => println!("Error communicating with pilot: {:?}", msg),
            Ok(command) => {
                if command.fire && self.fire_timer >= settings.rate_of_fire  {
                    self.fire_timer = 0.0;

                    let new_entity = entities.create();
                    make_bullet(
                        specs::world::LazyBuilder {
                            entity: new_entity,
                            lazy: &*lazy,
                        },
                        firing_position,
                        settings.bullet_speed,
                        command.fire_bearing,
                        &mut collision_world,
                    );
                }
            }
        }
    }
}
