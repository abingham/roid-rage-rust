use nalgebra::{Point2, Vector2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Roid {
    pub id: u32,
    pub radius: f32,
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    /// The field in which we're playing
    pub field: crate::core::field::Field,

    /// The point from which a bullet will be fired
    pub firing_position: Point2<f32>,

    // TODO: Could just model bullet as velocity (bearing + speed)
    /// The bearing from which a bullet will be fired
    pub firing_bearing: f32,

    /// The speed of the bullet    
    pub bullet_speed: f32,

    /// The time left until a bullet may be fired
    pub time_to_fire: f32,

    /// All of the roids in the game
    pub roids: Vec<Roid>,
}

/// Firing and movement command returned by the pilot
#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    /// Whether or not to fire
    pub fire: bool,

    /// Rotation command: +1 = clockwise, 0 = none/stop, -1 = counterclockwise
    pub rotation: i8,
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