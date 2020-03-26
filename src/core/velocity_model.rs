use crate::types::velocity::{from_speed_and_bearing, Velocity};
use nalgebra::{Point2, Vector2};
use std::collections::HashMap;

// TODO: This should probably just be made into a component.

struct TrackingData {
    start: Point2<f32>,
    end: Point2<f32>,
    duration: f32,
}

enum State {
    Initiated(Point2<f32>),
    Tracked(TrackingData),
}

pub struct VelocityModel {
    objects: HashMap<specs::world::Index, State>,
}

impl VelocityModel {
    pub fn new() -> VelocityModel {
        VelocityModel {
            objects: HashMap::<specs::world::Index, State>::new(),
        }
    }

    pub fn update<'a, I>(&mut self, source: I, time_delta: f32) -> ()
    where
        I: Iterator<Item = (specs::world::Index, Point2<f32>)>,
    {
        let updates = source.map(|(id, position)| {
            let state = match self.objects.get(&id) {
                None => State::Initiated(position),
                Some(State::Initiated(p1)) => State::Tracked(TrackingData {
                    start: *p1,
                    end: position,
                    duration: time_delta,
                }),
                Some(State::Tracked(tracked)) => {
                    let cur_vec = tracked.end - tracked.start;
                    let new_vec = position - tracked.end;

                    if cur_vec.bearing() != new_vec.bearing() {
                        State::Initiated(position)
                    } else {
                        State::Tracked(TrackingData {
                            start: tracked.start,
                            end: position,
                            duration: tracked.duration + time_delta,
                        })
                    }
                }
            };
            (id, state)
        });

        self.objects = updates.collect();
    }

    pub fn velocity(&self, id: specs::world::Index) -> Option<Vector2<f32>> {
        match self.objects.get(&id) {
            Some(State::Tracked(tracked)) => {
                let dvel = tracked.end - tracked.start;
                Some(from_speed_and_bearing(
                    dvel.speed() * tracked.duration,
                    dvel.bearing(),
                ))
            }
            _ => None,
        }
    }
}
