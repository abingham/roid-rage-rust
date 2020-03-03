use ncollide2d::world::CollisionWorld;
use ncollide2d::pipeline::CollisionObjectSlabHandle;
use crate::components::CollisionHandle;
use specs::{Join, World, BitSet, System, ReaderId, SystemData, ReadStorage, WriteStorage, WriteExpect, storage::ComponentEvent};

#[derive(Default)]
pub struct CollisionCleanupSystem {
    pub dirty: BitSet,
    pub reader_id: Option<ReaderId<ComponentEvent>>
}

/// Monitors removed CollisionHandles and removes them from the collision world
impl<'a> System<'a> for CollisionCleanupSystem {
    type SystemData = (
        ReadStorage<'a, CollisionHandle>,
        WriteExpect<'a, CollisionWorld::<f32, specs::world::Index>>,
    );

    fn run(&mut self, (collision_handles, mut collision_world): Self::SystemData) {
        self.dirty.clear();

        let events = collision_handles.channel().read(self.reader_id.as_mut().unwrap());

        for event in events {
            match event {
                ComponentEvent::Removed(id) => {
                    self.dirty.add(*id);
                },
                _ => {}
            }
        }

        // TODO: This doesn't work because the collision handles have already been removed and thus we can't iterate
        // over them.
        let mut removals: Vec<CollisionObjectSlabHandle> = vec![];
        for (collision_handle, _) in (&collision_handles, &self.dirty).join() {
            removals.push(collision_handle.0);
        }
        collision_world.remove(&removals);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(
            WriteStorage::<CollisionHandle>::fetch(&world).register_reader()
        );
    }
}