use crate::components::CollisionHandle;
use ncollide2d::pipeline::CollisionObjectSlabHandle;
use ncollide2d::world::CollisionWorld;
use specs::{
    storage::ComponentEvent, BitSet, ReadStorage, ReaderId, System, SystemData, World, WriteExpect,
    WriteStorage,
};
use std::collections::HashSet;

#[derive(Default)]
pub struct CleanupCollisionsSystem {
    pub dirty: BitSet,
    pub reader_id: Option<ReaderId<ComponentEvent>>,
}

/// Monitors removed CollisionHandles and removes them from the collision world
impl<'a> System<'a> for CleanupCollisionsSystem {
    type SystemData = (
        ReadStorage<'a, CollisionHandle>,
        WriteExpect<'a, CollisionWorld<f32, specs::world::Index>>,
    );

    fn run(&mut self, (collision_handles, mut collision_world): Self::SystemData) {
        self.dirty.clear();

        let events = collision_handles
            .channel()
            .read(self.reader_id.as_mut().unwrap());
        let removed: HashSet<specs::world::Index> = events
            .filter_map(|event| match event {
                ComponentEvent::Removed(id) => Some(*id),
                _ => None,
            })
            .collect();

        let removed_handles: Vec<CollisionObjectSlabHandle> = collision_world
            .collision_objects()
            .filter_map(|(handle, obj)| {
                if removed.contains(obj.data()) {
                    Some(handle)
                } else {
                    None
                }
            })
            .collect();

        if !removed_handles.is_empty() {
            println!("removed: {:?}", removed_handles);
        }
        collision_world.remove(&removed_handles);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(WriteStorage::<CollisionHandle>::fetch(&world).register_reader());
    }
}
