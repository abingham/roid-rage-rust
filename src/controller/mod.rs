use crate::model::Model;

// pub mod basic_controller;
// pub mod targeting;
// pub mod velocity_model;

pub trait Controller {
    fn update(&mut self, time_delta: f64);
    fn model(&self) -> &Model;
}

