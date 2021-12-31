/// This queries the pilot process using grpc to figure out
/// what it wants to do, e.g. shoot, turn, etc.
use crate::components::{
    make_bullet, AngularVelocity, Bullet, LinearVelocity, Position, Roid, Rotation, Ship, TimeDelta,
};
use crate::core::field::Field;
use crate::core::util::from_quantity_and_bearing;
use crate::settings::Settings;
use glam::Vec2;
use ncollide2d::world::CollisionWorld;
use roid_rage_grpc::roid_rage as rpc;
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
            mut linear_velocities,
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

        let roids: Vec<rpc::Roid> = (&roids, &linear_velocities, &positions, &entities)
            .join()
            .map(|(roid, linear_velocity, position, entity)| rpc::Roid {
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

        for (ship, position, rotation, linear_velocity, angular_velocity) in (
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
                ship_center.x + rotation.0.radians().cos() * ship.length / 2.0,
                ship_center.y + rotation.0.radians().sin() * ship.length / 2.0,
            );

            let game_state = rpc::GameState {
                field: Some(rpc::Field {
                    width: field.width() as i32,
                    height: field.height() as i32,
                }),
                firing_position: Some(rpc::Position {
                    x: firing_position.x,
                    y: firing_position.y
                }),
                time_to_fire: settings.rate_of_fire - self.fire_timer,
                roids: roids.clone(),
                ship: Some(rpc::Ship {
                    mass: ship.mass,
                    thrust: ship.thrust,
                    position: Some(rpc::Position {
                        x: position.0.x,
                        y: position.0.y
                    }),
                    velocity: Some(rpc::Velocity {
                        x: linear_velocity.0.x,
                        y: linear_velocity.0.y
                    }),
                    heading: rotation.0.radians(),
                    cannon: Some(rpc::Cannon {
                        bullet_speed: ship.cannon.bullet_speed,
                        rate_of_fire: ship.cannon.rate_of_fire
                    })
                })
            };

            // TODO: We need to sort out async execution (e.g. with tokio) to do this.
            // Pass game-state to pilot process
            // let res = query_pilot(settings.pilot_url, game_state);

            // match res {
            //     Err(msg) => println!("Error communicating with pilot: {:?}", msg),
            //     Ok(command) => {
            //         if command.fire && self.fire_timer >= settings.rate_of_fire {
            //             self.fire_timer = 0.0;

            //             let new_entity = entities.create();
            //             make_bullet(
            //                 specs::world::LazyBuilder {
            //                     entity: new_entity,
            //                     lazy: &*lazy,
            //                 },
            //                 firing_position,
            //                 settings.bullet_speed,
            //                 rotation.0.radians(),
            //                 &mut collision_world,
            //             );
            //         }

            //         let rotation_direction = match command.rotation {
            //             pilot::Rotation::Clockwise => 1.0,
            //             pilot::Rotation::Counterclockwise => -1.0,
            //             pilot::Rotation::None => 0.0,
            //         };

            //         angular_velocity.0 = rotation_direction * ship.rotational_speed;

            //         if command.thrusters {
            //             let steering_force =
            //                 from_quantity_and_bearing(ship.thrust, rotation.0.radians());
            //             let accel = steering_force / ship.mass;
            //             linear_velocity.0 += accel * time_delta.0.as_secs_f32();
            //         }
            //     }
            // }
        }
    }
}

async fn query_pilot(url: String, game_state: rpc::GameState) -> Result<rpc::Command, Box<dyn std::error::Error>> {
    let mut client = rpc::pilot_client::PilotClient::connect(url).await?;

    // let request = tonic::Request::new(HelloRequest {
    //     name: "Tonic".into(),
    // });

    // let response = client.say_hello(request).await?;

    // println!("RESPONSE={:?}", response);

    Ok(rpc::Command {
        fire: false,
        rotation: 0, // TODO: Use enum somehow!    
        thrusters: false
    })
}