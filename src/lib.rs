#[macro_use] #[cfg(test)] extern crate approx;

use specs::{Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow};

pub mod components;
// pub mod collide;
// pub mod controller;
// pub mod model;
pub mod velocity;
// pub mod view;

