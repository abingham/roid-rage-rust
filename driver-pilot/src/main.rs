#![feature(proc_macro_hygiene, decl_macro)]

extern crate nalgebra;

use rocket::{catch, catchers, post, routes};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use roid_rage::core::pilot::{Command, GameState};
use roid_rage::core::velocity::Velocity;

#[post("/", format = "json", data = "<game_state>")]
fn update(game_state: Json<GameState>) -> Json<Command> {
    let cmd = Command {
        fire: false,
        rotation: 0,
        thrusters: false,
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