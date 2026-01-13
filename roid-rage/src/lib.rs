mod components;
pub mod core;
mod rendering;
pub mod settings;
mod systems;

use crate::core::field::Field;
use crate::systems::{
    AgeFragmentsSystem, CleanupCollisionsSystem, DetectCollisionsSystem, ExplodeBulletsSystem,
    ExplodeRoidsSystem, MoveObjectsSystem, QueryPilotSystem, RegisterPilotsSystem,
    RemoveOutOfBoundsSystem, RepopulateSystem, RespawnShipSystem, WrapObjectsSystem,
};
use sted::Velocity;

use crate::components::{
    Bullet, Fragment, LinearVelocity, Position, Roid, Rotation, Ship, TimeDelta,
};
use crate::rendering::Render;
use ggez::event::EventHandler;
use ggez::timer;
use ggez::{graphics, Context, GameResult};
use ggez::glam;
use ncollide2d::world::CollisionWorld;
use specs::prelude::*;
use specs::Join;
use std::time::Duration;

type Point2 = glam::Vec2;

struct Assets {
    font: String,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let font_name = "DejaVuSansMono";
        let font_data = graphics::FontData::from_path(ctx, "/DejaVuSansMono.ttf")?;
        ctx.gfx.add_font(font_name, font_data);

        Ok(Assets {
            font: font_name.to_string(),
        })
    }
}

pub struct RoidRage {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
    assets: Assets,
}

impl RoidRage {
    pub fn new(ctx: &mut Context, settings: settings::Settings) -> GameResult<RoidRage> {
        let mut world = World::new();

        world.insert(Field::new(
            settings.screen_width as usize,
            settings.screen_height as usize,
        ));
        world.insert(CollisionWorld::<f32, specs::world::Index>::new(0.02f32));
        world.insert(TimeDelta(Duration::from_secs(0)));
        world.insert(settings);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        world.insert(runtime);

        let query_pilot_system = QueryPilotSystem::new()?;

        let mut dispatcher = DispatcherBuilder::new()
            // TODO: Rename this to collision-system-maintenance or something
            .with(RegisterPilotsSystem::new(), "pilot-registration", &[])
            .with(
                CleanupCollisionsSystem::default(),
                "cleanup_collisions",
                &[],
            )
            .with(RepopulateSystem, "repopulate", &["cleanup_collisions"])
            .with(
                RespawnShipSystem,
                "respawn",
                &["pilot-registration", "cleanup_collisions"],
            )
            .with(AgeFragmentsSystem, "age_fragments", &[])
            .with(
                MoveObjectsSystem,
                "move_objects",
                &["repopulate", "respawn"],
            )
            .with(
                DetectCollisionsSystem,
                "detect_collisions",
                &["move_objects"],
            )
            .with(WrapObjectsSystem, "wrap_objects", &["detect_collisions"])
            .with(
                RemoveOutOfBoundsSystem,
                "remove_out_of_bounds",
                &["detect_collisions"],
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
                query_pilot_system,
                "query_pilot",
                &["wrap_objects", "remove_out_of_bounds"],
            )
            // .with(LoggingSystem, "logging", &["out_of_bounds"])
            .build();

        dispatcher.setup(&mut world);

        let assets = Assets::new(ctx)?;

        // Load/create resources such as images here.
        Ok(RoidRage {
            world: world,
            dispatcher: dispatcher,
            assets: assets,
        })
    }
}
impl EventHandler<ggez::GameError> for RoidRage {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
            {
                let mut delta = self.world.write_resource::<TimeDelta>();
                *delta = TimeDelta(ctx.time.delta());
            }
            self.dispatcher.dispatch(&mut self.world);
            self.world.maintain();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // This adds a buffer around the edge of the screen so that roids don't teleport from one side to the next.
        let settings = self.world.read_resource::<settings::Settings>();
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        canvas.set_screen_coordinates(
            graphics::Rect::new(
                settings.maximum_roid_radius,
                settings.maximum_roid_radius,
                settings.screen_width - settings.maximum_roid_radius * 2.0,
                settings.screen_height - settings.maximum_roid_radius * 2.0,
            ),
        );

        for (position, rotation, roid) in (
            &self.world.read_storage::<Position>(),
            &self.world.read_storage::<Rotation>(),
            &self.world.read_storage::<Roid>(),
        )
            .join()
        {
            roid.render(position.0, rotation.0, ctx, &mut canvas)?;
        }

        for (position, bullet) in (
            &self.world.read_storage::<Position>(),
            &self.world.read_storage::<Bullet>(),
        )
            .join()
        {
            bullet.render(position.0, 0.0, ctx, &mut canvas)?;
        }

        for (position, rotation, ship) in (
            &self.world.read_storage::<Position>(),
            &self.world.read_storage::<Rotation>(),
            &self.world.read_storage::<Ship>(),
        )
            .join()
        {
            ship.render(position.0, rotation.0, ctx, &mut canvas)?;
        }

        for (position, rotation, linear_velocity, _ship) in (
            &self.world.read_storage::<Position>(),
            &self.world.read_storage::<Rotation>(),
            &self.world.read_storage::<LinearVelocity>(),
            &self.world.read_storage::<Ship>(),
        )
            .join()
        {
            // HUD
            let hud_font_size = 20.0;
            let hud_x = settings.maximum_roid_radius + 10.0;
            let hud_y = settings.maximum_roid_radius + 10.0;

            let mut position_text = graphics::Text::new(format!(
                "position: {} {}",
                position.0.x, position.0.y
            ));
            position_text
                .set_font(self.assets.font.clone())
                .set_scale(hud_font_size);
            canvas.draw(
                &position_text,
                graphics::DrawParam::new()
                    .dest(Point2::new(hud_x, hud_y))
                    .color(graphics::Color::WHITE),
            );

            let hud_y = hud_y + hud_font_size;
            let mut heading_text = graphics::Text::new(format!("heading: {}", rotation.0));
            heading_text
                .set_font(self.assets.font.clone())
                .set_scale(hud_font_size);
            canvas.draw(
                &heading_text,
                graphics::DrawParam::new()
                    .dest(Point2::new(hud_x, hud_y))
                    .color(graphics::Color::WHITE),
            );

            let hud_y = hud_y + hud_font_size;
            let mut bearing_text =
                graphics::Text::new(format!("bearing: {}", linear_velocity.0.bearing()));
            bearing_text
                .set_font(self.assets.font.clone())
                .set_scale(hud_font_size);
            canvas.draw(
                &bearing_text,
                graphics::DrawParam::new()
                    .dest(Point2::new(hud_x, hud_y))
                    .color(graphics::Color::WHITE),
            );

            let hud_y = hud_y + hud_font_size;
            let mut speed_text =
                graphics::Text::new(format!("speed: {}", linear_velocity.0.speed()));
            speed_text
                .set_font(self.assets.font.clone())
                .set_scale(hud_font_size);
            canvas.draw(
                &speed_text,
                graphics::DrawParam::new()
                    .dest(Point2::new(hud_x, hud_y))
                    .color(graphics::Color::WHITE),
            );
        }

        for (position, fragment) in (
            &self.world.read_storage::<Position>(),
            &self.world.read_storage::<Fragment>(),
        )
            .join()
        {
            fragment.render(position.0, 0.0, ctx, &mut canvas)?;
        }

        canvas.finish(ctx)?;

        timer::yield_now();

        Ok(())
    }
}
