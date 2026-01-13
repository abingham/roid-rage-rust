use nalgebra::{Point2, Vector2};
use std::collections::HashMap;

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
    objects: HashMap<u32, State>,
}

impl VelocityModel {
    pub fn new() -> VelocityModel {
        VelocityModel {
            objects: HashMap::new(),
        }
    }

    pub fn update<'a, I>(&mut self, source: I, time_delta: f32)
    where
        I: Iterator<Item = (u32, Point2<f32>)>,
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

                    if bearing(&cur_vec) != bearing(&new_vec) {
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

    pub fn velocity(&self, id: u32) -> Option<Vector2<f32>> {
        match self.objects.get(&id) {
            Some(State::Tracked(tracked)) => {
                let dvel = tracked.end - tracked.start;
                Some(from_speed_and_bearing(
                    speed(&dvel) * tracked.duration,
                    bearing(&dvel),
                ))
            }
            _ => None,
        }
    }
}

fn speed(vector: &Vector2<f32>) -> f32 {
    (vector.x * vector.x + vector.y * vector.y).sqrt()
}

fn bearing(vector: &Vector2<f32>) -> f32 {
    vector.y.atan2(vector.x)
}

fn from_speed_and_bearing(speed: f32, bearing: f32) -> Vector2<f32> {
    Vector2::new(speed * bearing.cos(), speed * bearing.sin())
}
