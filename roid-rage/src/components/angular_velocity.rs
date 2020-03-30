use specs::Component;
use specs::DenseVecStorage;

/// rotational rate
pub struct AngularVelocity(pub f32);

impl Component for AngularVelocity {
    type Storage = DenseVecStorage<Self>;
}
