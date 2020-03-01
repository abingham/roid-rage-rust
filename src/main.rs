// use amethyst::{
//     core::transform::TransformBundle,
//     prelude::*,
//     renderer::{
//         plugins::{RenderFlat3D, RenderFlat2D, RenderToWindow},
//         types::DefaultBackend,
//         RenderingBundle,
//     },
//     utils::application_root_dir,
// };

mod components;
mod field;
mod objects;
mod systems;

// use crate::roid_rage::RoidRage;
use crate::systems::{
    CollisionSystem, LoggingSystem, OutOfBoundsSystem, VelocitySystem, WrappingSystem,
};
use crate::field::Field;

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use specs::prelude::*;
use ncollide2d::world::CollisionWorld;
use crate::objects::make_roid;

fn main() {

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
		.build()
		.expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = RoidRage::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct RoidRage {
    world: World,
    dispatcher: Dispatcher<'static, 'static>
}

impl RoidRage {
    pub fn new(_ctx: &mut Context) -> RoidRage {
        let mut world = World::new();

        world.insert(Field::new(800, 600));
        world.insert(CollisionWorld::<f32, ()>::new(0.02f32));


        let mut dispatcher = DispatcherBuilder::new()
            .with(VelocitySystem, "velocity_system", &[])
            .with(CollisionSystem, "collision_system", &["velocity_system"])
            .with(WrappingSystem, "wrapping_system", &["collision_system"])
            .with(
                OutOfBoundsSystem,
                "out_of_bounds_system",
                &["wrapping_system"],
            )
            .with(LoggingSystem, "logging_system", &["out_of_bounds_system"])
            .build();

        dispatcher.setup(&mut world);

        println!("making roid");
        make_roid(&mut world, 400.0, 300.0);
        println!("made roid");

        // Load/create resources such as images here.
        RoidRage {
            world: world,
            dispatcher: dispatcher
        }
    }
}

impl EventHandler for RoidRage {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.dispatcher.dispatch(&mut self.world);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        graphics::present(ctx)
    }
}

// fn main() -> amethyst::Result<()> {
//     amethyst::start_logger(Default::default());

//     let app_root = application_root_dir()?;

//     let config_dir = app_root.join("config");
//     let display_config_path = config_dir.join("display.ron");

//     let game_data = GameDataBuilder::default()
//         .with_bundle(TransformBundle::new())?
//         .with(VelocitySystem, "velocity_system", &["transform_system"])
//         .with(CollisionSystem, "collision_system", &["velocity_system"])
//         .with(WrappingSystem, "wrapping_system", &["collision_system"])
//         .with(
//             OutOfBoundsSystem,
//             "out_of_bounds_system",
//             &["wrapping_system"],
//         )
//         .with(LoggingSystem, "logging_system", &["out_of_bounds_system"])
//         .with_bundle(
//             RenderingBundle::<DefaultBackend>::new()
//                 .with_plugin(
//                     RenderToWindow::from_config_path(display_config_path)
//                         .with_clear([0.34, 0.36, 0.52, 1.0]),
//                 )
//                 .with_plugin(RenderFlat3D::default()),
//         )?;

//     let mut game = Application::new("/", RoidRage, game_data)?;
//     game.run();

//     Ok(())
// }
