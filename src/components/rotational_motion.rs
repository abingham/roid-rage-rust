use nalgebra::Vector2;
use specs::Component;
use specs::DenseVecStorage;

/// rotational rate
pub struct RotationalMotion(pub f32);

impl Component for RotationalMotion {
    type Storage = DenseVecStorage<Self>;
}
