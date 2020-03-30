mod components;
pub mod core;
pub mod pilot;
mod rendering;
pub mod settings;
mod systems;

use crate::core::field::Field;
use crate::systems::{
    AgeFragmentsSystem, CleanupCollisionsSystem, DetectCollisionsSystem, ExplodeBulletsSystem,
    ExplodeRoidsSystem, FireOnTargetsSystem, LoggingSystem, MoveObjectsSystem,
    RemoveOutOfBoundsSystem, RepopulateSystem, WrapObjectsSystem,
};

use crate::components::{Bullet, Fragment, Roid, TimeDelta, Transform};
use crate::rendering::Render;
use ggez::event::EventHandler;
use ggez::timer;
use ggez::{graphics, Context, GameResult};
use ncollide2d::world::CollisionWorld;
use specs::prelude::*;
use specs::Join;
use std::time::Duration;

pub struct RoidRage {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl RoidRage {
    pub fn new(_ctx: &mut Context, settings: settings::Settings) -> RoidRage {
        let mut world = World::new();

        world.insert(Field::new(
            settings.screen_width as usize,
            settings.screen_height as usize,
        ));
        world.insert(CollisionWorld::<f32, specs::world::Index>::new(0.02f32));
        world.insert(TimeDelta(Duration::from_secs(0)));
        world.insert(settings);

        let mut dispatcher = DispatcherBuilder::new()
            // TODO: Rename this to collision-system-maintenance or something
            .with(
                CleanupCollisionsSystem::default(),
                "cleanup_collisions",
                &[],
            )
            .with(RepopulateSystem, "repopulate", &["cleanup_collisions"])
            .with(AgeFragmentsSystem, "age_fragments", &[])
            .with(MoveObjectsSystem, "move_objects", &["repopulate"])
            .with(
                DetectCollisionsSystem,
                "detect_collisions",
                &["move_objects"],
            )
            .with(WrapObjectsSystem, "wrap_objects", &["detect_collisions"])
            .with(
                RemoveOutOfBoundsSystem,
                "remove_out_of_bounds",
                &["wrap_objects"],
            )
            .with(
                ExplodeRoidsSystem,
                "explode_roids",
                &["remove_out_of_bounds"],
            )
            .with(
                ExplodeBulletsSystem,
                "explode_bullets",
                &["remove_out_of_bounds"],
            )
            .with(
                FireOnTargetsSystem::new(),
                "fire_on_targets",
                &["remove_out_of_bounds"],
            )
            // .with(LoggingSystem, "logging", &["out_of_bounds"])
            .build();

        dispatcher.setup(&mut world);

        // Load/create resources such as images here.
        RoidRage {
            world: world,
            dispatcher: dispatcher,
        }
    }
}

impl EventHandler for RoidRage {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS as u32) {
            {
                let mut delta = self.world.write_resource::<TimeDelta>();
                *delta = TimeDelta(timer::delta(ctx));
            }
            self.dispatcher.dispatch(&mut self.world);
            self.world.maintain();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // This adds a buffer around the edge of the screen so that roids don't teleport from one side to the next.
        let settings = self.world.read_resource::<settings::Settings>();
        graphics::set_screen_coordinates(
            ctx,
            graphics::Rect::new(
                settings.maximum_roid_radius,
                settings.maximum_roid_radius,
                settings.screen_width - settings.maximum_roid_radius * 2.0,
                settings.screen_height - settings.maximum_roid_radius * 2.0,
            ),
        )?;

        graphics::clear(ctx, graphics::BLACK);

        // TODO: Can we express the rendering as systems? This seems like the natural way to do things, but context
        // seems to get in the way.

        for (transform, roid) in (
            &self.world.read_storage::<Transform>(),
            &self.world.read_storage::<Roid>(),
        )
            .join()
        {
            roid.render(&transform, ctx)?;
        }

        for (transform, bullet) in (
            &self.world.read_storage::<Transform>(),
            &self.world.read_storage::<Bullet>(),
        )
            .join()
        {
            bullet.render(&transform, ctx)?;
        }

        for (transform, fragment) in (
            &self.world.read_storage::<Transform>(),
            &self.world.read_storage::<Fragment>(),
        )
            .join()
        {
            fragment.render(&transform, ctx)?;
        }

        graphics::present(ctx)?;

        timer::yield_now();

        Ok(())
    }
}
