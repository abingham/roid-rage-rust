mod components;
mod field;
mod systems;

use crate::field::Field;
use crate::systems::{
    CollisionDetectionSystem, ExplodeRoidsSystem, LoggingSystem, OutOfBoundsSystem, VelocitySystem,
    WrappingSystem,
};
use ggez::graphics::{Color, DrawMode, DrawParam};

use crate::components::{make_roid, Roid, TimeDelta, Transform};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ncollide2d::world::CollisionWorld;
use specs::prelude::*;
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
        world.insert(CollisionWorld::<f32, ()>::new(0.02f32));
        world.insert(TimeDelta(Duration::from_secs(0)));

        let mut dispatcher = DispatcherBuilder::new()
            .with(VelocitySystem, "velocity", &[])
            .with(
                CollisionDetectionSystem,
                "collision_detection",
                &["velocity"],
            )
            .with(WrappingSystem, "wrapping", &["collision_detection"])
            .with(OutOfBoundsSystem, "out_of_bounds", &["wrapping"])
            .with(ExplodeRoidsSystem, "explode_roids", &["out_of_bounds"])
            .with(LoggingSystem, "logging", &["out_of_bounds"])
            .build();

        dispatcher.setup(&mut world);

        let (vel, xform, w, chandle, roid) = make_roid(
            400.0,
            300.0,
            100.0,
            0.0,
            40.0,
            world.get_mut::<CollisionWorld<f32, ()>>().unwrap(),
        );
        world
            .create_entity()
            .with(vel)
            .with(xform)
            .with(w)
            .with(chandle)
            .with(roid)
            .build();

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
                println!("time delta: {}", timer::delta(ctx).as_secs_f32());
            }
            self.dispatcher.dispatch(&mut self.world);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

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

        graphics::present(ctx)?;

        timer::yield_now();

        Ok(())
    }
}
