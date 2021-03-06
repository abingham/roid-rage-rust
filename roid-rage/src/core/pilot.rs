use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Roid {
    pub id: u32,
    pub radius: f32,
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ship {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub angular_velocity: f32,
    pub heading: f32, // TODO: Express this as a Bearing. Will need some JSON work, I guess.
    pub acceleration: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    /// The field in which we're playing
    pub field: crate::core::field::Field,

    /// The point from which a bullet will be fired
    pub firing_position: Vector2<f32>,

    /// The bearing and speed of the bullet
    pub firing_velocity: Vector2<f32>,

    /// The time left until a bullet may be fired
    pub time_to_fire: f32,

    /// All of the roids in the game
    pub roids: Vec<Roid>,

    pub ship: Ship,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Rotation {
    Clockwise,
    Counterclockwise,
    None,
}

/// Firing and movement command returned by the pilot
#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    /// Whether or not to fire
    pub fire: bool,

    /// How to rotate
    pub rotation: Rotation,

    /// Whether to fire the thrusters (i.e. accelerate in current heading)
    pub thrusters: bool,
}

pub fn query_pilot(url: &str, game_state: &GameState) -> Result<Command, String> {
    let client = reqwest::blocking::Client::new();
    let cmd = client
        .post(url)
        .json(game_state)
        .send()
        .or_else(|e| Err(format!("{:?}", e)))?
        .json::<Command>()
        .or_else(|e| Err(format!("{:?}", e)))?;
    Ok(cmd)
}
