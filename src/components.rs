use nalgebra::{Point2, Vector2};
use specs::{FlaggedStorage, Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow};
use ncollide2d::shape::ShapeHandle;

pub struct Position {
    pub pos: Point2<f64>
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position {
            pos: Point2::<f64>::new(x, y)
        }
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
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

pub struct CollisionShape {
    pub handle: ShapeHandle<f64>
}

impl CollisionShape {
    pub fn new(handle: ShapeHandle<f64>) -> CollisionShape {
        CollisionShape {
            handle: handle
        }
    }
}

impl Component for CollisionShape {
    type Storage = VecStorage<Self>;
}