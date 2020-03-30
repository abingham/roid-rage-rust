use crate::components::{make_bullet, Bullet, LinearVelocity, Roid, TimeDelta, Transform};
use crate::core::field::Field;
use crate::core::targeting::find_target;
use crate::settings::Settings;
use nalgebra::Point2;
use ncollide2d::world::CollisionWorld;
use specs::{
    Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage,
};

pub struct FireOnTargetsSystem {
    time_since_last: f32,
}

impl FireOnTargetsSystem {
    pub fn new() -> FireOnTargetsSystem {
        FireOnTargetsSystem {
            time_since_last: 0.0,
        }
    }
}

/// Identify target and shoot a bullet
impl<'s> System<'s> for FireOnTargetsSystem {
    type SystemData = (
        ReadStorage<'s, Roid>,
        ReadStorage<'s, LinearVelocity>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Bullet>,
        ReadExpect<'s, Field>,
        Read<'s, TimeDelta>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, specs::world::Index>>,
        ReadExpect<'s, Settings>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            roids,
            linear_motions,
            transforms,
            _bullets,
            field,
            time_delta,
            entities,
            mut collision_world,
            settings,
            lazy,
        ): Self::SystemData,
    ) {
        self.time_since_last += time_delta.0.as_secs_f32();
        if self.time_since_last <= settings.rate_of_fire {
            return;
        }

        let targets = (&roids, &linear_motions, &transforms).join().map(
            |(_roid, linear_motion, transform)| {
                let pos = Point2::<f32>::from(transform.0.translation.vector);
                (pos, linear_motion.0)
            },
        );

        let firing_position =
            Point2::<f32>::new(settings.screen_width / 2.0, settings.screen_height / 2.0);
        find_target(&firing_position, settings.bullet_speed, &*field, targets).map(
            |target_bearing| {
                self.time_since_last = 0.0;

                let new_entity = entities.create();
                make_bullet(
                    specs::world::LazyBuilder {
                        entity: new_entity,
                        lazy: &*lazy,
                    },
                    firing_position,
                    settings.bullet_speed,
                    target_bearing,
                    &mut collision_world,
                );
            },
        );
    }
}
