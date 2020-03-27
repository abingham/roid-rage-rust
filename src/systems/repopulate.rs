use crate::components::{make_roid, Roid};
use crate::core::util::random_bearing;
use crate::settings::Settings;
use ncollide2d::world::CollisionWorld;
use specs::{Entities, LazyUpdate, Read, ReadStorage, System, WriteExpect};

/// Repopulate the fields with roids if there are none.
pub struct RepopulateSystem {
    // TODO: Should settings instead be a resource?
    settings: Settings,
}

impl RepopulateSystem {
    pub fn new(settings: Settings) -> RepopulateSystem {
        RepopulateSystem { settings: settings }
    }
}

/// Explode roids that have collided with something.
impl<'s> System<'s> for RepopulateSystem {
    type SystemData = (
        ReadStorage<'s, Roid>,
        Entities<'s>,
        WriteExpect<'s, CollisionWorld<f32, specs::world::Index>>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (roids, entities, mut collision_world, lazy): Self::SystemData) {
        if !roids.is_empty() {
            return;
        }

        use rand::prelude::*;
        let mut rng = thread_rng();
        for _ in 0..self.settings.initial_roid_count {
            let x =
                rng.gen::<f32>() * (self.settings.screen_width + self.settings.maximum_roid_radius);
            let y = rng.gen::<f32>()
                * (self.settings.screen_height + self.settings.maximum_roid_radius);
            let speed = rng.gen::<f32>()
                * (self.settings.max_initial_roid_speed - self.settings.min_initial_roid_speed)
                + self.settings.min_initial_roid_speed;
            let bearing = random_bearing();
            let radius = rng.gen::<f32>() * 5.0 + (self.settings.maximum_roid_radius - 5.0);
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
                self.settings.roid_bumpiness,
                &mut collision_world,
            );
        }
    }
}
