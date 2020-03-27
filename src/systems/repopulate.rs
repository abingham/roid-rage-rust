use crate::components::{make_roid, Roid};
use crate::core::util::random_bearing;
use crate::settings::Settings;
use ncollide2d::world::CollisionWorld;
use specs::{Entities, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect};

/// Repopulate the fields with roids if there are none.
pub struct RepopulateSystem;

impl<'s> System<'s> for RepopulateSystem {
    type SystemData = (
        ReadStorage<'s, Roid>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, specs::world::Index>>,
        ReadExpect<'s, Settings>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (roids, entities, mut collision_world, settings, lazy): Self::SystemData) {
        if !roids.is_empty() {
            return;
        }

        use rand::prelude::*;
        let mut rng = thread_rng();
        for _ in 0..settings.initial_roid_count {
            let x =
                rng.gen::<f32>() * (settings.screen_width + settings.maximum_roid_radius);
            let y = rng.gen::<f32>()
                * (settings.screen_height + settings.maximum_roid_radius);
            let speed = rng.gen::<f32>()
                * (settings.max_initial_roid_speed - settings.min_initial_roid_speed)
                + settings.min_initial_roid_speed;
            let bearing = random_bearing();
            let radius = rng.gen::<f32>() * 5.0 + (settings.maximum_roid_radius - 5.0);
            let angular_velocity = rng.gen::<f32>() * 0.005 + 0.005;
            let new_entity = entities.create();
            make_roid(
                specs::world::LazyBuilder {
                    entity: new_entity,
                    lazy: &*lazy,
                },
                x,
                y,
                speed,
                bearing,
                angular_velocity,
                radius,
                settings.roid_bumpiness,
                &mut collision_world,
            );
        }
    }
}
