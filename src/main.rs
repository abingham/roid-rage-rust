mod components;
mod core;
mod rendering;
mod systems;

use crate::core::field::Field;
use crate::core::util::random_bearing;
use crate::systems::{
    AgeFragmentsSystem, CleanupCollisionsSystem, DetectCollisionsSystem, ExplodeBulletsSystem,
    ExplodeRoidsSystem, FireOnTargetsSystem, LoggingSystem, MoveObjectsSystem,
    RemoveOutOfBoundsSystem, WrapObjectsSystem,
};

use crate::components::{make_roid, Bullet, Fragment, Roid, TimeDelta, Transform};
use crate::rendering::Render;
use ggez::event::{self, EventHandler};
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ncollide2d::world::CollisionWorld;
use specs::prelude::*;
use specs::Join;
use std::time::Duration;

use ggez::conf;

const MAX_ROID_RADIUS: f32 = 42.5;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Roid Rage!", "Austin Bingham")
        .window_setup(conf::WindowSetup::default().title("Roid Rage!"))
        .window_mode(conf::WindowMode::default().dimensions(
            SCREEN_WIDTH + MAX_ROID_RADIUS * 2.0,
            SCREEN_HEIGHT + MAX_ROID_RADIUS * 2.0,
        ))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler. Usually, you should provide it with the Context object to use when
    // setting your game up.
    let mut my_game = RoidRage::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct RoidRage {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl RoidRage {
    pub fn new(_ctx: &mut Context) -> RoidRage {
        let mut world = World::new();

        world.insert(Field::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize));
        world.insert(CollisionWorld::<f32, specs::world::Index>::new(0.02f32));
        world.insert(TimeDelta(Duration::from_secs(0)));

        let mut dispatcher = DispatcherBuilder::new()
            // TODO: Rename this to collision-system-maintenance or something
            .with(
                CleanupCollisionsSystem::default(),
                "cleanup_collisions",
                &[],
            )
            .with(AgeFragmentsSystem, "age_fragments", &[])
            .with(MoveObjectsSystem, "move_objects", &[])
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
                ExplodeRoidsSystem::new(15.0),
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

        make_some_roids(&mut world);

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
        graphics::set_screen_coordinates(
            ctx,
            graphics::Rect::new(
                MAX_ROID_RADIUS,
                MAX_ROID_RADIUS,
                SCREEN_WIDTH - MAX_ROID_RADIUS * 2.0,
                SCREEN_HEIGHT - MAX_ROID_RADIUS * 2.0,
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

fn make_some_roids(world: &mut World) {
    use rand::prelude::*;
    let mut rng = thread_rng();
    for _ in 0..10 {
        let x = rng.gen::<f32>() * (SCREEN_WIDTH + MAX_ROID_RADIUS);
        let y = rng.gen::<f32>() * (SCREEN_HEIGHT + MAX_ROID_RADIUS);
        let speed = rng.gen::<f32>() * 50.0 + 50.0;
        let bearing = random_bearing();
        let radius = rng.gen::<f32>() * 5.0 + (MAX_ROID_RADIUS - 5.0);
        let angular_velocity = rng.gen::<f32>() * 0.005 + 0.005;

        let entity = world.write_resource::<specs::world::EntitiesRes>().create();

        make_roid(
            specs::world::LazyBuilder {
                entity: entity,
                lazy: &*world.read_resource::<specs::world::LazyUpdate>(),
            },
            x,
            y,
            speed,
            bearing,
            angular_velocity,
            radius,
            &mut world.write_resource::<CollisionWorld<f32, specs::world::Index>>(),
        );
    }
}
