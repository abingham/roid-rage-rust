use nalgebra::Vector2;
use specs::Component;
use specs::VecStorage;

pub struct LinearVelocity(pub Vector2<f32>);

impl Component for LinearVelocity {
    type Storage = VecStorage<Self>;
}
