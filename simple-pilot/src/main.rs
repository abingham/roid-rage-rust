#![feature(proc_macro_hygiene, decl_macro)]

mod targeting;

extern crate nalgebra;

use rocket::{catch, catchers, post, routes};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use roid_rage::pilot::{Command, GameState};
use crate::targeting::find_target;

#[post("/", format = "json", data = "<game_state>")]
fn update(game_state: Json<GameState>) -> Json<Command> {
    let target = find_target(
        &nalgebra::Point2::<f32>::new(game_state.firing_position.x, game_state.firing_position.y),
        game_state.bullet_speed,
        &game_state.field,
        &game_state.roids,
    );

    let cmd = match target {
        Some(bearing) => {
            Command {
                fire: true,
                fire_bearing: bearing,
            }
        },
        None => {
            Command {
                fire: false,
                fire_bearing: 0.0,
            }
        }
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
