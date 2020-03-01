use super::collision_groups::{ROID_GROUP, WEAPON_GROUP};
use crate::components::{CollisionHandle, Velocity, Wrapping};
use amethyst::assets::AssetLoaderSystemData;
use amethyst::core::Transform;
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::renderer::rendy::mesh::{Normal, Position, TexCoord};
// use amethyst::renderer::MaterialDefaults;
use nalgebra::{zero, Isometry2, Vector2};
use ncollide2d::pipeline::{CollisionGroups, GeometricQueryType};
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::world::CollisionWorld;
// use amethyst::renderer::palette::LinSrgba;
// use amethyst::renderer::loaders;
// use amethyst::renderer::rendy::texture::Texture;
use amethyst::renderer::Mesh;
use amethyst::assets::Handle;
// use amethyst_rendy::loaders;
// use amethyst_rendy::palette::rgb::LinSrgba;
use amethyst::{
    // assets::{Loader},
    // prelude::*,
    renderer::{
        // rendy::mesh::{MeshBuilder},
        // types::MeshData,
        // ImageFormat, Material,
    },
};

pub fn make_roid(world: &mut World, x: f32, y: f32) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.0);

    let mut collision_groups = CollisionGroups::new();
    collision_groups.set_membership(&[WEAPON_GROUP]);
    collision_groups.set_whitelist(&[ROID_GROUP]);

    let collision_isometry = Isometry2::new(Vector2::new(x as f64, y as f64), zero());

    let radius: f64 = 10.0;

    let collision_shape = ShapeHandle::new(Ball::new(radius));

    // Put entry in collision world
    let collision_world: &mut CollisionWorld<f64, ()> =
        world.get_mut::<CollisionWorld<f64, ()>>().unwrap();

    let (collision_handle, _) = collision_world.add(
        collision_isometry,
        collision_shape,
        collision_groups,
        GeometricQueryType::Contacts(0.0, 0.0),
        (),
    );

    let mesh = _render(world);

    // Create a roid entity
    world
        .create_entity()
        .with(Velocity::new(10.0, 10.0))
        .with(transform)
        .with(Wrapping)
        .with(CollisionHandle::new(collision_handle))
        .with(mesh)
        // .with(material)
        .build();
}

fn _render<'a>(world: &mut World) -> Handle<Mesh> {
    // Here's a hint as to how to render dynamically.
    //https://www.reddit.com/r/rust_gamedev/comments/c3s9wi/need_help_creating_a_mesh_programmatically_with/

    // let default_mat = world.read_resource::<MaterialDefaults>().0.clone();

    let vertices = vec![
        Position([0., 0., 0.]),
        Position([1., 0., 0.]),
        Position([0., 1., 0.]),
    ];

    let norm = vec![
        Normal([0., 0., 1.]),
        Normal([0., 0., 1.]),
        Normal([0., 0., 1.]),
    ];

    let tex = vec![TexCoord([0., 0.]), TexCoord([1., 1.]), TexCoord([0., 1.])];

    let mesh = world.exec(
        |loader: AssetLoaderSystemData<amethyst::renderer::types::Mesh>| {
            loader.load_from_data(
                amethyst::renderer::types::MeshData(
                    amethyst::renderer::rendy::mesh::MeshBuilder::new()
                        .with_vertices(vertices)
                        .with_vertices(norm)
                        .with_vertices(tex), // .with_indices(Indices::U16(/* vec of u16 */.into()))
                ),
                (),
            )
        },
    );

    // let albedo = world.exec(|loader: AssetLoaderSystemData<Texture>| {
    //     loader.load_from_data(
    //         loaders::load_from_linear_rgba(LinSrgba::new(1.0, 0.0, 0.0, 1.0)).into(),
    //         (),
    //     )
    // });

    // let mat = world.exec(|loader: AssetLoaderSystemData<Material>| {
    //     loader.load_from_data(
    //         Material {
    //             albedo,
    //             ..default_mat.clone()
    //         },
    //         (),
    //     )
    // });

    mesh
}
