#![feature(proc_macro_hygiene, decl_macro)]

mod targeting;

extern crate nalgebra;

use rocket::{catch, catchers, post, routes};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use roid_rage::pilot::GameState;
use crate::targeting::find_target;

#[post("/", format = "json", data = "<game_state>")]
fn update(game_state: Json<GameState>) -> JsonValue {
    find_target(
        &nalgebra::Point2::<f32>::new(game_state.firing_position.0, game_state.firing_position.1),
        game_state.bullet_speed,
        &game_state.field,
        &game_state.roids,
    );
    json!({
        "status": "coolio...",
    })
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
        .mount("/update", routes![update])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
