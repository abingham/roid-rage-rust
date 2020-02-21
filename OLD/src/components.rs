use nalgebra::{Point2, Vector2};
use specs::{FlaggedStorage, Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow};
use ncollide2d::pipeline::CollisionObjectSlabHandle;

pub struct Position {
    pub pos: Point2<f64>
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position {
            pos: Point2::<f64>::new(x, y)
        }
    }

    pub fn x(&self) -> f64 {
        self.pos.x
    }

    pub fn y(&self) -> f64 {
        self.pos.y
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct Velocity {
    pub vel: Vector2<f64>
}

impl Velocity {
    pub fn new(x: f64, y: f64) -> Velocity {
        Velocity {
            vel: Vector2::<f64>::new(x, y)
        }
    }

    pub fn from_speed_bearing(speed: f64, bearing: f64) -> Velocity {
        Velocity {
            vel: Vector2::<f64>::new(bearing.cos(), bearing.sin()) * speed
        }
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

pub struct CollisionHandle(pub CollisionObjectSlabHandle);

impl Component for CollisionHandle {
    type Storage = VecStorage<Self>;
}
