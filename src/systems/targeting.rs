use ncollide2d::world::CollisionWorld;
use crate::components::{make_bullet, Bullet, Roid, Transform, TimeDelta};
use specs::{ReadStorage, Entities, WriteStorage, Read, LazyUpdate, System, WriteExpect};

pub struct TargetingSystem {
    time_since_last: f32
}

impl TargetingSystem {
    pub fn new() -> TargetingSystem {
        TargetingSystem {
            time_since_last: 0.0
        }
    }
}

/// Identify target and shoot a bullet
impl<'s> System<'s> for TargetingSystem {
    type SystemData = (
        ReadStorage<'s, Roid>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Bullet>,
        Read<'s, TimeDelta>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, ()>>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (_roids, _transforms, _bullets, time_delta, entities, mut collision_world, lazy): Self::SystemData) {
        self.time_since_last += time_delta.0.as_secs_f32();
        
        // Fire once every second
        if self.time_since_last > 1.0 {
            self.time_since_last = 0.0;

            let (vel, xform, chandle, bullet) = make_bullet(400.0, 300.0, 0.0, &mut collision_world);

            let new_entity = entities.create();
            lazy.insert(new_entity, bullet);
            lazy.insert(new_entity, vel);
            lazy.insert(new_entity, xform);
            lazy.insert(new_entity, chandle);
        }
    }
}
