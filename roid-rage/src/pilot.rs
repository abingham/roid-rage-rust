use serde::{Deserialize, Serialize};
use nalgebra::{Vector2, Point2};

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y}
    }
}

impl From<Point2::<f32>> for Point {
    fn from(p: Point2<f32>) -> Point {
        Point::new(p.x, p.y)
    }
}

impl From<Vector2::<f32>> for Point {
    fn from(v: Vector2<f32>) -> Point {
        Point::new(v.x, v.y)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Roid {
    pub id: u32,
    pub radius: f32,
    pub position: Point,
    pub velocity: Point,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub field: crate::core::field::Field,
    pub firing_position: Point,
    pub bullet_speed: f32,
    pub roids: Vec<Roid>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub fire: bool,
    pub fire_bearing: f32,
}

pub fn query_pilot(address: &str, game_state: &GameState) -> Result<Command, String> {
    let url = format!("http://{}/update", address);
    let client = reqwest::blocking::Client::new();
    let cmd = client.post(&url)
        .json(game_state)
        .send()
        .or_else(|e| Err(format!("{:?}", e)))?
        .json::<Command>()
        .or_else(|e| Err(format!("{:?}", e)))?;   
    Ok(cmd)
}