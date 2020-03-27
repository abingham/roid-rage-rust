use crate::components::{make_bullet, Bullet, LinearVelocity, Roid, TimeDelta, Transform};
use crate::core::field::Field;
use crate::core::targeting::find_target;
use nalgebra::Point2;
use ncollide2d::world::CollisionWorld;
use specs::{
    Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage,
};

pub struct FireOnTargetsSystem {
    rate_of_fire: f32,
    time_since_last: f32,
    firing_position: Point2<f32>,
    bullet_speed: f32,
}

impl FireOnTargetsSystem {
    pub fn new(
        rate_of_fire: f32,
        firing_position: Point2<f32>,
        bullet_speed: f32,
    ) -> FireOnTargetsSystem {
        FireOnTargetsSystem {
            rate_of_fire: rate_of_fire,
            time_since_last: 0.0,
            firing_position: firing_position,
            bullet_speed: bullet_speed,
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
            lazy,
        ): Self::SystemData,
    ) {
        self.time_since_last += time_delta.0.as_secs_f32();
        if self.time_since_last <= self.rate_of_fire {
            return;
        }

        let targets = (&roids, &linear_motions, &transforms).join().map(
            |(_roid, linear_motion, transform)| {
                let pos = Point2::<f32>::from(transform.0.translation.vector);
                (pos, linear_motion.0)
            },
        );

        find_target(&self.firing_position, self.bullet_speed, &*field, targets).map(
            |target_bearing| {
                self.time_since_last = 0.0;

                let new_entity = entities.create();
                make_bullet(
                    specs::world::LazyBuilder {
                        entity: new_entity,
                        lazy: &*lazy,
                    },
                    self.firing_position,
                    self.bullet_speed,
                    target_bearing,
                    &mut collision_world,
                );
            },
        );
    }
}
