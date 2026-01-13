/// This queries the pilot process using grpc to figure out
/// what it wants to do, e.g. shoot, turn, etc.
use crate::components::{
    make_bullet, AngularVelocity, Bullet, LinearVelocity, Pilot, Position, Roid, Rotation, Ship,
    TimeDelta,
};
use crate::core::field::Field;
use crate::settings::Settings;
use glam::Vec2;
use ncollide2d::world::CollisionWorld;
use roid_rage_grpc::roid_rage as rpc;
use specs::{
    Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage,
};
use std::convert::TryFrom;
use sted::to_vector;

pub struct QueryPilotSystem {
    // TODO: I think this should be a per-pilot component. This will allow use to
    // a) have one per pilot (important!) and b) zero it when there's a bullet collision.
    fire_timer: f32,
}

impl QueryPilotSystem {
    pub fn new() -> Result<QueryPilotSystem, std::io::Error> {
        Ok(QueryPilotSystem { fire_timer: 0.0 })
    }
}

/// Identify target and shoot a bullet
impl<'s> System<'s> for QueryPilotSystem {
    type SystemData = (
        ReadStorage<'s, Roid>,
        ReadStorage<'s, Pilot>,
        ReadStorage<'s, Ship>,
        WriteStorage<'s, LinearVelocity>,
        WriteStorage<'s, AngularVelocity>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Rotation>,
        WriteStorage<'s, Bullet>,
        ReadExpect<'s, Field>,
        Read<'s, TimeDelta>,
        ReadExpect<'s, tokio::runtime::Runtime>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, specs::world::Index>>,
        ReadExpect<'s, Settings>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            roids,
            pilots,
            ships,
            mut linear_velocities,
            mut angular_velocities,
            positions,
            rotations,
            _bullets,
            field,
            time_delta,
            runtime,
            entities,
            mut collision_world,
            settings,
            lazy,
        ): Self::SystemData,
    ) {
        self.fire_timer += time_delta.0.as_secs_f32();

        let roids: Vec<rpc::Roid> = (&roids, &linear_velocities, &positions)
            .join()
            .map(|(roid, linear_velocity, position)| rpc::Roid {
                radius: roid.radius,
                position: Some(rpc::Position {
                    x: position.0.x,
                    y: position.0.y,
                }),
                velocity: Some(rpc::Velocity {
                    x: linear_velocity.0.x,
                    y: linear_velocity.0.y,
                }),
            })
            .collect();

        for (pilot, ship, position, rotation, linear_velocity, angular_velocity) in (
            &pilots,
            &ships,
            &positions,
            &rotations,
            &mut linear_velocities,
            &mut angular_velocities,
        )
            .join()
        {
            let ship_center = position.0;

            let firing_position = Vec2::new(
                // TODO: Would it be better to use rotation.0.normalized() instead of doing trig here?
                ship_center.x + rotation.0.cos() * ship.length / 2.0,
                ship_center.y + rotation.0.sin() * ship.length / 2.0,
            );

            let game_state = rpc::GameState {
                field: Some(rpc::Field {
                    width: field.width() as i32,
                    height: field.height() as i32,
                }),
                firing_position: Some(rpc::Position {
                    x: firing_position.x,
                    y: firing_position.y,
                }),
                time_to_fire: settings.rate_of_fire - self.fire_timer,
                roids: roids.clone(),
                ship: Some(rpc::Ship {
                    mass: ship.mass,
                    thrust: ship.thrust,
                    position: Some(rpc::Position {
                        x: position.0.x,
                        y: position.0.y,
                    }),
                    velocity: Some(rpc::Velocity {
                        x: linear_velocity.0.x,
                        y: linear_velocity.0.y,
                    }),
                    heading: rotation.0,
                    cannon: Some(rpc::Cannon {
                        bullet_speed: ship.cannon.bullet_speed,
                        rate_of_fire: ship.cannon.rate_of_fire,
                    }),
                }),
            };

            // Pass game-state to pilot process
            let res = runtime.block_on(query_pilot(pilot.url.to_string(), game_state));

            match res {
                // TODO: If we fail to reach a pilot after some time, we should remove it and its ship.
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
                            to_vector(rotation.0).normalize() * settings.bullet_speed,
                            &mut collision_world,
                        );
                    }

                    let rotation_direction = match rpc::Rotation::try_from(command.rotation) {
                        Ok(rpc::Rotation::Clockwise) => 1.0,
                        Ok(rpc::Rotation::Counterclockwise) => -1.0,
                        Ok(rpc::Rotation::None) => 0.0,
                        Err(_) => 0.0, // TODO: Should raise an error? Assert?
                    };

                    angular_velocity.0 = rotation_direction * ship.rotational_speed;

                    if command.thrusters {
                        let steering_force = ship.thrust * to_vector(rotation.0);
                        let accel = steering_force / ship.mass;
                        linear_velocity.0 += accel * time_delta.0.as_secs_f32();
                    }
                }
            }
        }
    }
}

async fn query_pilot(
    url: String,
    game_state: rpc::GameState,
) -> Result<rpc::Command, Box<dyn std::error::Error>> {
    let mut client = rpc::pilot_client::PilotClient::connect(url).await?;

    let request = tonic::Request::new(game_state);

    let response = client.get_command(request).await?;

    Ok(response.get_ref().clone())
}
