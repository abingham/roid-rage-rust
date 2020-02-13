use std::collections::HashMap;
use nalgebra::{Point2, Vector2};
use uuid::Uuid;
use crate::model::game_object::GameObject;
use crate::velocity::{make_velocity_vector, Velocity};

enum State {
    Single(Point2<f64>),
    Pair(Point2<f64>, Point2<f64>, f64)
}

pub struct VelocityModel {
    objects: HashMap<Uuid, State>
}

impl VelocityModel {
    pub fn new() -> VelocityModel {
        VelocityModel{
            objects: HashMap::<Uuid, State>::new()
        }
    }

    pub fn update<'a, I>(&mut self, objects: I, time_delta: f64) -> () 
        where I: Iterator<Item = &'a dyn GameObject>
    {
        let updates = objects
            .map(|obj| {
                let state = match self.objects.get(&obj.id()) {
                    None => {
                        State::Single(*obj.position())
                    },
                    Some(State::Single(p1)) => {
                        State::Pair(*p1, *obj.position(), time_delta)
                    },
                    Some(State::Pair(p1, p2, ptd)) => {
                        let cur_vec = p2 - p1;
                        let new_vec = obj.position() - p2;

                        if cur_vec.bearing() != new_vec.bearing() {
                            State::Single(*obj.position())
                        }
                        else {
                            State::Pair(*p1, *obj.position(), ptd + time_delta)
                        }
                    }
                };
                (obj.id(), state)
            });
        
        self.objects = updates.collect();
    }

    pub fn velocity(&self, id: Uuid) -> Option<Vector2<f64>> {
        match self.objects.get(&id) {
            Some(State::Pair(p1, p2, time_delta)) => {
                let dvel = p2 - p1;
                Some(make_velocity_vector(dvel.speed() * time_delta, dvel.bearing()))
            },
            _ => {
                None
            }
        }
    }
}