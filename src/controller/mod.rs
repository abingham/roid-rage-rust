pub mod velocity_model;
pub mod targeting;

use crate::model::Model;
use crate::model::objects::bullet::Bullet;
use nalgebra::Point2;
use velocity_model::VelocityModel;
use targeting::target;

pub trait Controller {
    fn update(&mut self, time_delta: f64);
    fn model(&self) -> &Model;
}

pub struct BasicController {
    model: Model,
    full_time: f64,
    vmodel: VelocityModel
}

const FIRING_FREQUENCY: f64 = 0.5;

impl BasicController {
    pub fn new(model: Model) -> BasicController {
        BasicController {
            model: model,
            full_time: 0.0,
            vmodel: VelocityModel::new(),
        }
    }

    fn update(&mut self, time_delta: f64) {
        self.model.project(time_delta);
        self.vmodel.update(self.model.objects(), time_delta);
        self.fire(time_delta);
    }
    
    fn fire(&mut self, dt: f64) -> () {
        let firing_position = Point2::new(
            (self.model.field().width() / 2) as f64,
            (self.model.field().height() / 2) as f64,
        );

        self.full_time += dt;
        if self.full_time > FIRING_FREQUENCY {
            let target_bearing = target( &firing_position, Bullet::speed(), 
                                         self.model.field(),
                                         self.model.objects(),
                                         &self.vmodel);
            if let Some(bearing) = target_bearing {
                self.full_time = 0.0;
                let bullet = Bullet::new(firing_position, bearing);
                self.model.insert(Box::new(bullet));
            }
        }
    }
}

impl Controller for BasicController {
    fn update(&mut self, time_delta: f64) {
        // TODO: Does this recurse or behave as I'm hoping?
        self.update(time_delta);
    }

    fn model(&self) -> &Model {
        &self.model
    }
}