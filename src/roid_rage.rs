use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use nalgebra::Vector2;

pub const FIELD_HEIGHT: f32 = 600.0;
pub const FIELD_WIDTH: f32 = 800.0;

pub struct RoidRage;

impl SimpleState for RoidRage {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        
        world.register::<Velocity>();

        initialise_camera(world);
        initialise_roids(world);
    }
}

pub struct Velocity {
    pub vec: Vector2<f64>
}

impl Velocity {
    pub fn new(dx: f64, dy: f64) -> Velocity {
        Velocity {
            vec: Vector2::<f64>::new(dx, dy)
        }
    }

    pub fn from_speed_and_bearing(speed: f64, bearing: f64) -> Velocity {
        Velocity {
            vec: Vector2::<f64>::new(bearing.cos(), bearing.sin()) * speed
        }
    }

    pub fn speed(&self) -> f64 {
        (self.vec.x.powf(2.0) + self.vec.y.powf(2.0)).sqrt()
    }

    pub fn bearing(&self) -> f64 {
        self.vec.y.atan2(self.vec.x)
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
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