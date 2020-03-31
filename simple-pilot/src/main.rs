#![feature(proc_macro_hygiene, decl_macro)]

mod targeting;

extern crate nalgebra;

use crate::targeting::find_target;
use rocket::{catch, catchers, post, routes};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use roid_rage::core::pilot::{Command, GameState};

#[post("/", format = "json", data = "<game_state>")]
fn update(game_state: Json<GameState>) -> Json<Command> {
    let target = find_target(
        &nalgebra::Point2::<f32>::new(game_state.firing_position.x, game_state.firing_position.y),
        game_state.bullet_speed,
        &game_state.field,
        &game_state.roids,
    );

    let cmd = match target {
        Some(_bearing) => Command {
            fire: true,
            rotation: 0,
        },
        None => Command {
            fire: false,
            rotation: 0,
        },
    };

    Json(cmd)
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![update])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
