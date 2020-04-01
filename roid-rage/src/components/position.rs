use nalgebra::Point2;
use specs::Component;
use specs::VecStorage;

// TODO: Perhaps this should be a vector? Many (all?) uses of position want it as a vector, not a point.
#[derive(Clone)]
pub struct Position(pub Point2<f32>);

impl Component for Position {
    type Storage = VecStorage<Self>;
}
