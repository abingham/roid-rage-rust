use amethyst::ecs::prelude::Component;
use amethyst::ecs::storage::DenseVecStorage;
use nalgebra::Vector2;
use ncollide2d::pipeline::CollisionObjectSlabHandle;

pub struct CollisionHandle {
    pub handle: CollisionObjectSlabHandle,
}

impl CollisionHandle {
    pub fn new(handle: CollisionObjectSlabHandle) -> Self {
        CollisionHandle {
            handle: handle,
        }
    }
}

impl Component for CollisionHandle {
    type Storage = DenseVecStorage<Self>;
}
