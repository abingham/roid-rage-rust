use ncollide2d::pipeline::CollisionObjectSlabHandle;
use specs::{storage::FlaggedStorage, Component, DenseVecStorage};

pub struct CollisionHandle(pub CollisionObjectSlabHandle);

impl Component for CollisionHandle {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
