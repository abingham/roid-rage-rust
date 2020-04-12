#![feature(proc_macro_hygiene, decl_macro)]

extern crate nalgebra;

use rocket::{catch, catchers, post, routes, State};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use roid_rage::core::pilot::{Command, GameState};
use std::f32::consts::PI;
use std::sync::Mutex;
use float_cmp::ApproxEqRatio;

enum Activity {
    Thrust(usize),
    Rotate(f32),
}

struct PilotState {
    activity: Activity,
}

// Note:
// Change in x under constant rotational velocity (r) and constant thrust (a) for time (t) and final heading h(t):
//    Vx(0) * t + a / 2 * (t^2 * sin(h(t)) + 2 * t * cos(h(t)) - 2 * sin(h(t)))

#[post("/", format = "json", data = "<game_state>")]
fn update(game_state: Json<GameState>, pilot_state: State<Mutex<PilotState>>) -> Json<Command> {
    let mut cmd = Command {
        fire: false,
        rotation: 0,
        thrusters: false,
    };

    let mut pilot_state = pilot_state.lock().unwrap();

    match &pilot_state.activity {
        Activity::Thrust(counter) => {
            if *counter == 0 {
                let new_heading = (game_state.ship.heading + PI) % (PI * 2.0);
                pilot_state.activity = Activity::Rotate(new_heading);
            } else {
                pilot_state.activity = Activity::Thrust(counter - 1);
                cmd.thrusters = true;
            }
        }
        Activity::Rotate(target) => {
            if game_state.ship.heading.approx_eq_ratio(target, 0.005) {
                println!("target = {}, heading = {}", *target, game_state.ship.heading);
                pilot_state.activity = Activity::Thrust(600);
            }
            else {
                cmd.rotation = 1;
            }
        }
    }

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
        .manage(Mutex::<PilotState>::new(PilotState {
            activity: Activity::Thrust(600),
        }))
}

fn main() {
    rocket().launch();
}
