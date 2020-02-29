use amethyst::{core::Transform, prelude::*, renderer::Camera};
use ncollide2d::world::CollisionWorld;

use crate::components::Velocity;
use crate::components::Wrapping;
use crate::field::Field;
use crate::objects::make_roid;

pub struct RoidRage;

impl SimpleState for RoidRage {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // TODO: Is this the right place to put this? How can we read in the field dimension from the config file?
        world.insert(Field::new(800, 600));
        world.insert(CollisionWorld::<f64, ()>::new(0.02f64));

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

    make_roid(world, width / 2.0, height / 2.0);
}
