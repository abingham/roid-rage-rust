use std::collections::HashMap;
use nalgebra::{Point2, Vector2};
use uuid::Uuid;
use crate::model::game_object::GameObject;
use crate::velocity::{make_velocity_vector, Velocity};

struct TrackingData {
    start: Point2<f64>,
    end: Point2<f64>,
    duration: f64
}

enum State {
    Initiated(Point2<f64>),
    Tracked(TrackingData)
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
                        State::Initiated(*obj.position())
                    },
                    Some(State::Initiated(p1)) => {
                        State::Tracked(TrackingData {
                            start: *p1, 
                            end: *obj.position(), 
                            duration: time_delta
                        })
                    },
                    Some(State::Tracked(tracked)) => {
                        let cur_vec = tracked.end - tracked.start;
                        let new_vec = obj.position() - tracked.end;

                        if cur_vec.bearing() != new_vec.bearing() {
                            State::Initiated(*obj.position())
                        }
                        else {
                            State::Tracked(TrackingData {
                                start: tracked.start, 
                                end: *obj.position(), 
                                duration: tracked.duration + time_delta
                            })
                        }
                    }
                };
                (obj.id(), state)
            });
        
        self.objects = updates.collect();
    }

    pub fn velocity(&self, id: Uuid) -> Option<Vector2<f64>> {
        match self.objects.get(&id) {
            Some(State::Tracked(tracked)) => {
                let dvel = tracked.end - tracked.start;
                Some(make_velocity_vector(dvel.speed() * tracked.duration, dvel.bearing()))
            },
            _ => {
                None
            }
        }
    }
}