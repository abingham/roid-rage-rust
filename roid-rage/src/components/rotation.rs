use specs::Component;
use specs::VecStorage;
use std::f32::consts::PI;

// TODO: Can I make this into a number type with modular behavior?
#[derive(Clone)]
pub struct Rotation {
    r: f32
}

impl Rotation {
    pub fn new(radians: f32) -> Rotation {
        Rotation {
            r: radians
        }
    }

    pub fn radians(&self) -> f32 {
        self.r
    }

    pub fn inc(&mut self, delta: f32) {
        self.r += delta;
        self.r = self.r % (PI * 2.0); 
    }
}

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}
