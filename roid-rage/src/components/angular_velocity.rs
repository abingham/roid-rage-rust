use specs::Component;
use specs::VecStorage;

/// rotational rate
pub struct AngularVelocity(pub f32);

impl Component for AngularVelocity {
    type Storage = VecStorage<Self>;
}
