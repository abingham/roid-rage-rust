use amethyst::{core::Transform, prelude::*, renderer::Camera};

use crate::components::Velocity;
use crate::components::Wrapping;
use crate::field::Field;

pub struct RoidRage;

impl SimpleState for RoidRage {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // TODO: Is this the right place to put this? How can we read in the field dimension from the config file?
        world.insert(Field::new(800, 600));

        initialise_camera(world);
        initialise_roids(world);
    }
}

fn initialise_camera(world: &mut World) {
    let (width, height) = world
        .try_fetch::<Field>()
        .map(|f| (f.width() as f32, f.height() as f32))
        .unwrap();

    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}

fn initialise_roids(world: &mut World) {
    let (width, height) = world
        .try_fetch::<Field>()
        .map(|f| (f.width() as f32, f.height() as f32))
        .unwrap();

    let mut transform = Transform::default();

    // Put the roid in the middle of the field
    transform.set_translation_xyz(width / 2.0, height / 2.0, 0.0);

    // Create a roid entity
    world
        .create_entity()
        .with(Velocity::new(1.0, 0.0))
        .with(transform)
        .with(Wrapping)
        .build();
}
