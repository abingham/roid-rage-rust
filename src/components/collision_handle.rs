use specs::Component;
use specs::DenseVecStorage;
use ncollide2d::pipeline::CollisionObjectSlabHandle;

pub struct CollisionHandle(pub CollisionObjectSlabHandle);

impl Component for CollisionHandle {
    type Storage = DenseVecStorage<Self>;
}
