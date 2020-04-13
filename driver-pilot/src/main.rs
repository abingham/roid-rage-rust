#![feature(proc_macro_hygiene, decl_macro)]

extern crate nalgebra;

use float_cmp::ApproxEqRatio;
use rand::prelude::*;
use rocket::{catch, catchers, post, routes, State};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use sted::Bearing;
use roid_rage::core::pilot::{Command, GameState, Rotation};
use sted::Velocity;
use std::f32::consts::PI;
use std::sync::Mutex;

enum Activity {
    Accelerate(usize),
    Stop,
}

struct PilotState {
    activity: Activity,
}

fn stop_ship(game_state: &GameState) -> Command {
    let heading = Bearing::new(game_state.ship.heading);
    let course = Bearing::new(game_state.ship.velocity.bearing());
    let diff = heading.distance(&course);

    let mut cmd = Command {
        fire: false,
        rotation: Rotation::None,
        thrusters: false,
    };

    // If we're not facing opposite to our motion, keep rotating to get there.
    if !diff.approx_eq_ratio(&PI, 0.01) {
        cmd.rotation = if diff.signum() as i8 > 0 {
            Rotation::Counterclockwise
        } else {
            Rotation::Clockwise
        };
    }
    // If we're still moving, fire thrusters
    else {
        cmd.thrusters = true;
    }

    cmd
}

// Note:
// Change in x under constant rotational velocity (r) and constant thrust (a) for time (t) and final heading h(t):
//    Vx(0) * t + a / 2 * (t^2 * sin(h(t)) + 2 * t * cos(h(t)) - 2 * sin(h(t)))

#[post("/", format = "json", data = "<game_state>")]
fn update(game_state: Json<GameState>, pilot_state: State<Mutex<PilotState>>) -> Json<Command> {
    let mut cmd = Command {
        fire: false,
        rotation: Rotation::None,
        thrusters: false,
    };

    let mut pilot_state = pilot_state.lock().unwrap();

    match &pilot_state.activity {
        Activity::Accelerate(counter) => {
            if *counter == 0 {
                pilot_state.activity = Activity::Stop;
            } else {
                pilot_state.activity = Activity::Accelerate(counter - 1);
                cmd.thrusters = true;
            }
        }
        Activity::Stop => {
            if game_state.ship.velocity.speed() < 0.1 {
                let mut rng = rand::thread_rng();
                let num_frames = rng.next_u32() % 300 + 300;
                pilot_state.activity = Activity::Accelerate(num_frames as usize);
            } else {
                cmd = stop_ship(&*game_state);
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
            activity: Activity::Accelerate(90),
        }))
}

fn main() {
    rocket().launch();
}
