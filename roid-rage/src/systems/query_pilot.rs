/// This queries the pilot process using grpc to figure out
/// what it wants to do, e.g. shoot, turn, etc.
use crate::components::{
    make_bullet, AngularVelocity, Bullet, FireTimer, LinearVelocity, Pilot, Position, Roid,
    Rotation, Ship, TimeDelta,
};
use crate::core::field::Field;
use crate::settings::Settings;
use ncollide2d::world::CollisionWorld;
use roid_rage_grpc::roid_rage as rpc;
use specs::{
    Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage,
};
use std::convert::TryFrom;
use sted::to_vector;

pub struct QueryPilotSystem;

impl QueryPilotSystem {
    pub fn new() -> Result<QueryPilotSystem, std::io::Error> {
        Ok(QueryPilotSystem)
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
        WriteStorage<'s, FireTimer>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Rotation>,
        WriteStorage<'s, Bullet>,
        ReadExpect<'s, Field<f32>>,
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
            mut fire_timers,
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

        let mut disconnected = Vec::new();

        for (
            entity,
            pilot,
            ship,
            fire_timer,
            position,
            rotation,
            linear_velocity,
            angular_velocity,
        ) in (
            &entities,
            &pilots,
            &ships,
            &mut fire_timers,
            &positions,
            &rotations,
            &mut linear_velocities,
            &mut angular_velocities,
        )
            .join()
        {
            fire_timer.0 += time_delta.0.as_secs_f32();

            let ship_center = position.0;

            let heading = to_vector(rotation.0);
            let firing_position = ship_center + heading * (ship.length / 2.0);

            let game_state = rpc::GameState {
                field: Some(rpc::Field {
                    width: field.width() as i32,
                    height: field.height() as i32,
                }),
                firing_position: Some(rpc::Position {
                    x: firing_position.x,
                    y: firing_position.y,
                }),
                // TODO: Is time_to_fire actually used?
                time_to_fire: settings.rate_of_fire - fire_timer.0,
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
                Err(msg) => {
                    println!("Error communicating with pilot {}: {:?}", pilot.url, msg);
                    disconnected.push(entity);
                }
                Ok(command) => {
                    if apply_command(
                        &command,
                        ship,
                        &rotation,
                        linear_velocity,
                        angular_velocity,
                        fire_timer,
                        &time_delta,
                        &settings,
                        &pilot.url,
                    ) {
                        let new_entity = entities.create();
                        make_bullet(
                            specs::world::LazyBuilder {
                                entity: new_entity,
                                lazy: &*lazy,
                            },
                            firing_position,
                            heading * settings.bullet_speed,
                            &mut collision_world,
                        );
                    }
                }
            }
        }

        for entity in disconnected {
            if let Err(err) = entities.delete(entity) {
                println!("Failed to remove unresponsive pilot entity: {:?}", err);
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

fn apply_command(
    command: &rpc::Command,
    ship: &Ship,
    rotation: &Rotation,
    linear_velocity: &mut LinearVelocity,
    angular_velocity: &mut AngularVelocity,
    fire_timer: &mut FireTimer,
    time_delta: &TimeDelta,
    settings: &Settings,
    pilot_url: &str,
) -> bool {
    let heading = to_vector(rotation.0);
    let mut fired = false;

    if command.fire && fire_timer.0 >= settings.rate_of_fire {
        fire_timer.0 = 0.0;
        fired = true;
    }

    let rotation_direction = match rpc::Rotation::try_from(command.rotation) {
        Ok(rpc::Rotation::Clockwise) => Some(1.0),
        Ok(rpc::Rotation::Counterclockwise) => Some(-1.0),
        Ok(rpc::Rotation::None) => Some(0.0),
        Err(_) => {
            println!(
                "Invalid rotation value {} from pilot {}",
                command.rotation, pilot_url
            );
            None
        }
    };

    if let Some(rotation_direction) = rotation_direction {
        angular_velocity.0 = rotation_direction * ship.rotational_speed;
    }

    if command.thrusters {
        let steering_force = ship.thrust * heading;
        let accel = steering_force / ship.mass;
        linear_velocity.0 += accel * time_delta.0.as_secs_f32();
    }

    fired
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> Settings {
        Settings::load().expect("settings")
    }

    fn ship() -> Ship {
        Ship {
            mass: 2.0,
            thrust: 4.0,
            length: 1.0,
            width: 1.0,
            rotational_speed: 3.0,
            cannon: crate::components::Cannon {
                bullet_speed: 10.0,
                rate_of_fire: 0.5,
            },
        }
    }

    #[test]
    fn apply_command_sets_rotation() {
        let mut linear_velocity = LinearVelocity(glam::Vec2::ZERO);
        let mut angular_velocity = AngularVelocity(0.0);
        let mut fire_timer = FireTimer(0.0);
        let rotation = Rotation(0.0);
        let time_delta = TimeDelta(std::time::Duration::from_secs_f32(0.5));
        let command = rpc::Command {
            fire: false,
            rotation: rpc::Rotation::Clockwise as i32,
            thrusters: false,
        };

        apply_command(
            &command,
            &ship(),
            &rotation,
            &mut linear_velocity,
            &mut angular_velocity,
            &mut fire_timer,
            &time_delta,
            &settings(),
            "pilot",
        );

        assert!((angular_velocity.0 - 3.0).abs() < 0.0001);
    }

    #[test]
    fn apply_command_ignores_invalid_rotation() {
        let mut linear_velocity = LinearVelocity(glam::Vec2::ZERO);
        let mut angular_velocity = AngularVelocity(0.5);
        let mut fire_timer = FireTimer(0.0);
        let rotation = Rotation(0.0);
        let time_delta = TimeDelta(std::time::Duration::from_secs_f32(0.5));
        let command = rpc::Command {
            fire: false,
            rotation: 99,
            thrusters: false,
        };

        apply_command(
            &command,
            &ship(),
            &rotation,
            &mut linear_velocity,
            &mut angular_velocity,
            &mut fire_timer,
            &time_delta,
            &settings(),
            "pilot",
        );

        assert!((angular_velocity.0 - 0.5).abs() < 0.0001);
    }

    #[test]
    fn apply_command_thrusters_accelerate_along_heading() {
        let mut linear_velocity = LinearVelocity(glam::Vec2::ZERO);
        let mut angular_velocity = AngularVelocity(0.0);
        let mut fire_timer = FireTimer(0.0);
        let rotation = Rotation(0.0);
        let time_delta = TimeDelta(std::time::Duration::from_secs_f32(0.5));
        let command = rpc::Command {
            fire: false,
            rotation: rpc::Rotation::None as i32,
            thrusters: true,
        };

        apply_command(
            &command,
            &ship(),
            &rotation,
            &mut linear_velocity,
            &mut angular_velocity,
            &mut fire_timer,
            &time_delta,
            &settings(),
            "pilot",
        );

        assert!((linear_velocity.0.x - 1.0).abs() < 0.0001);
        assert!((linear_velocity.0.y - 0.0).abs() < 0.0001);
    }

    #[test]
    fn apply_command_fires_when_ready() {
        let mut linear_velocity = LinearVelocity(glam::Vec2::ZERO);
        let mut angular_velocity = AngularVelocity(0.0);
        let mut fire_timer = FireTimer(1.0);
        let rotation = Rotation(0.0);
        let time_delta = TimeDelta(std::time::Duration::from_secs_f32(0.5));
        let command = rpc::Command {
            fire: true,
            rotation: rpc::Rotation::None as i32,
            thrusters: false,
        };

        let fired = apply_command(
            &command,
            &ship(),
            &rotation,
            &mut linear_velocity,
            &mut angular_velocity,
            &mut fire_timer,
            &time_delta,
            &settings(),
            "pilot",
        );

        assert!(fired);
        assert!((fire_timer.0 - 0.0).abs() < 0.0001);
    }
}
