mod components;
mod field;
mod systems;
mod util;

use crate::field::Field;
use crate::systems::{
    AgeFragmentsSystem, CollisionCleanupSystem, CollisionDetectionSystem, ExplodeBulletsSystem,
    ExplodeRoidsSystem, LoggingSystem, OutOfBoundsSystem, TargetingSystem, VelocitySystem,
    WrappingSystem,
};
use crate::util::random_bearing;
use ggez::graphics::{Color, DrawMode, DrawParam};

use crate::components::{make_roid, Bullet, Fragment, Roid, TimeDelta, Transform};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ncollide2d::world::CollisionWorld;
use specs::prelude::*;
use specs::world::Index;
use specs::Join;
use std::time::Duration;

use ggez::conf;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Roid Rage", "Austin Bingham")
        .window_setup(conf::WindowSetup::default().title("Roid Rage!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
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

        world.insert(Field::new(800, 600));
        world.insert(CollisionWorld::<f32, specs::world::Index>::new(0.02f32));
        world.insert(TimeDelta(Duration::from_secs(0)));

        let mut dispatcher = DispatcherBuilder::new()
            // TODO: Rename this to collision-system-maintenance or something
            .with(CollisionCleanupSystem::default(), "collision_cleanup", &[])
            .with(AgeFragmentsSystem, "age_fragments", &[])
            .with(VelocitySystem, "velocity", &[])
            .with(
                CollisionDetectionSystem,
                "collision_detection",
                &["velocity"],
            )
            .with(WrappingSystem, "wrapping", &["collision_detection"])
            .with(OutOfBoundsSystem, "out_of_bounds", &["wrapping"])
            .with(ExplodeRoidsSystem, "explode_roids", &["out_of_bounds"])
            .with(ExplodeBulletsSystem, "explode_bullets", &["out_of_bounds"])
            .with(TargetingSystem::new(), "targeting", &["out_of_bounds"])
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
        const DESIRED_FPS: u32 = 30;

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
        graphics::clear(ctx, graphics::BLACK);

        // TODO: Can we express the rendering as systems? This seems like the natural way to do things, but context
        // seems to get in the way.

        for (transform, roid) in (
            &self.world.read_storage::<Transform>(),
            &self.world.read_storage::<Roid>(),
        )
            .join()
        {
            let mb = &mut graphics::MeshBuilder::new();
            mb.circle(
                DrawMode::fill(),
                Point2::<f32>::new(
                    transform.0.translation.vector.x,
                    transform.0.translation.vector.y,
                ),
                roid.radius,
                0.1,
                Color::new(1.0, 1.0, 1.0, 1.0),
            );
            let mesh = mb.build(ctx)?;
            graphics::draw(ctx, &mesh, DrawParam::new())?;
        }

        for (transform, _bullet) in (
            &self.world.read_storage::<Transform>(),
            &self.world.read_storage::<Bullet>(),
        )
            .join()
        {
            let mb = &mut graphics::MeshBuilder::new();
            mb.circle(
                DrawMode::fill(),
                Point2::<f32>::new(
                    transform.0.translation.vector.x,
                    transform.0.translation.vector.y,
                ),
                Bullet::radius(),
                0.1,
                Color::new(1.0, 1.0, 1.0, 1.0),
            );
            let mesh = mb.build(ctx)?;
            graphics::draw(ctx, &mesh, DrawParam::new())?;
        }

        for (transform, _fragment) in (
            &self.world.read_storage::<Transform>(),
            &self.world.read_storage::<Fragment>(),
        )
            .join()
        {
            let mb = &mut graphics::MeshBuilder::new();
            mb.circle(
                DrawMode::fill(),
                Point2::<f32>::new(
                    transform.0.translation.vector.x,
                    transform.0.translation.vector.y,
                ),
                Fragment::radius(),
                0.1,
                Color::new(1.0, 1.0, 1.0, 1.0),
            );
            let mesh = mb.build(ctx)?;
            graphics::draw(ctx, &mesh, DrawParam::new())?;
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
        let x = rng.gen::<f32>() * 800.0;
        let y = rng.gen::<f32>() * 600.0;
        let speed = rng.gen::<f32>() * 50.0 + 50.0;
        let bearing = random_bearing();
        let radius = rng.gen::<f32>() * 5.0 + 37.5;

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
            radius,
            &mut world.fetch_mut::<CollisionWorld<f32, specs::world::Index>>(),
        );
    }
}
