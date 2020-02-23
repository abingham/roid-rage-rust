use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{Transform},
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use nalgebra::Vector2;
use crate::components::Velocity;

pub const FIELD_HEIGHT: f32 = 600.0;
pub const FIELD_WIDTH: f32 = 800.0;

pub struct RoidRage;

impl SimpleState for RoidRage {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        
        initialise_camera(world);
        initialise_roids(world);
    }
}


fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    transform.set_translation_xyz(FIELD_WIDTH * 0.5, FIELD_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(FIELD_WIDTH, FIELD_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_roids(world: &mut World) {
    let mut transform = Transform::default();

    // Put the roid in the middle of the field
    transform.set_translation_xyz(FIELD_WIDTH / 2.0, FIELD_HEIGHT / 2.0, 0.0);

    // Create a roid entity
    world
        .create_entity()
        .with(Velocity::new(1.0, 0.0))
        .with(transform)
        .build();
}