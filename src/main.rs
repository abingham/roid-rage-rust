use amethyst::{
    core::transform::TransformBundle,
    ecs::prelude::{ReadExpect, SystemData},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod components;
mod roid_rage;
mod systems;

use crate::roid_rage::RoidRage;
use crate::systems::VelocitySystem;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let mut world = World::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(VelocitySystem, "velocity", &["transform_system"])
        ;

    let mut game = Application::new("/", RoidRage, game_data)?;
    game.run();

    Ok(())
}
