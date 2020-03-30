#[macro_use] extern crate serde_derive;
use std::collections::HashMap;

fn main() {
    let response = make_request();
    match response {
        Ok(state) => println!("Ok! {:?}", state),
        Err(msg) => println!("Err :( {:?}", msg)
    }
}

fn make_request() -> Result<HashMap<String, String>, String> {
    let field = Field { width: 800, height: 600};
    let state = GameState {
        field: field,
        firing_position: (400.0, 300.0),
        bullet_speed: 1000.0,
        roids: vec![]
    };

    let client = reqwest::blocking::Client::new();
    let data = client.post("http://127.0.0.1:8000/update")
        .json(&state)
        .send()
        .or_else(|e| Err(format!("{:?}", e)))?
        .json::<HashMap<String, String>>()
        .or_else(|e| Err(format!("{:?}", e)))?;

    Ok(data)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Roid {
    id: u32,
    radius: f32,
    position: (f32, f32),
    velocity: (f32, f32)
}

#[derive(Serialize, Deserialize, Debug)]
struct GameState {
    field: Field,
    firing_position: (f32, f32),
    bullet_speed: f32,
    roids: Vec<Roid>
}