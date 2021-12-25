use ggez::conf;
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use std::path;

fn main() -> GameResult {
    let settings =
        roid_rage::settings::Settings::load().expect("Unable to load Roid Rage settings!");

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Roid Rage!", "Austin Bingham")
        .window_setup(conf::WindowSetup::default().title("Roid Rage!"))
        .window_mode(conf::WindowMode::default().dimensions(
            settings.screen_width + settings.maximum_roid_radius * 2.0,
            settings.screen_height + settings.maximum_roid_radius * 2.0,
        ))
        .add_resource_path(path::PathBuf::from("./resources"))
        .build()
        .expect("Ermahgerd, could not create ggez context!");

    // Create an instance of your event handler. Usually, you should provide it with the Context object to use when
    // setting your game up.
    let my_game = roid_rage::RoidRage::new(&mut ctx, settings)?;

    // Run!
    event::run(ctx, event_loop, my_game)
}
