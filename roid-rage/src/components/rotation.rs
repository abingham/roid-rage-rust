use specs::Component;
use specs::VecStorage;
use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Rotation {
    r: f32,
}

impl Rotation {
    pub fn new(radians: f32) -> Rotation {
        Rotation { r: 0.0 } + radians
    }

    pub fn radians(&self) -> f32 {
        self.r
    }
}

impl std::ops::Add<f32> for Rotation {
    type Output = Rotation;

    fn add(self, rhs: f32) -> Self::Output {
        let mut r = self.clone();
        r += rhs;
        r
    }
}

impl std::ops::AddAssign<f32> for Rotation {
    fn add_assign(&mut self, rhs: f32) {
        self.r = (self.r + rhs) % (PI * 2.0);
    } 
}

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}
