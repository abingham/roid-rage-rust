use serde::{Deserialize, Serialize};
use nalgebra::Point2;

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub width: usize,
    pub height: usize,
}

impl Field {
    pub fn contains(&self, p: &Point2<f32>) -> bool {
        p.x >= 0.0 && p.x <= self.width as f32 && p.y >= 0.0 && p.y <= self.height as f32 
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Roid {
    pub id: u32,
    pub radius: f32,
    pub position: (f32, f32),
    pub velocity: (f32, f32)
}
#[derive(Serialize, Deserialize, Debug)]
struct GameState {
    field: Field,
    firing_position: (f32, f32),
    bullet_speed: f32,
    roids: Vec<Roid>
}


