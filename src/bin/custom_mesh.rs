/// Create a custom mesh by manually passing its attributes. Add a light source and a camera, and
/// position them such that the lit mesh is visible from the camera.

use bevy::prelude::*;
use bevy::render::mesh::*;
use bevy::render::render_resource::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create and add a default material
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.7, 0.6, 0.7),
        ..Default::default()
    });

    // Location to put cube
    let mesh_location = Vec3::new(0.0, 0.0, 0.0);


    // Create a `Mesh` by specifying its attributes
//     let custom_mesh = Mesh::from(shape::Cube::new(0.7));
    let mut custom_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    custom_mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.7, 0.0, 0.0], [0.0, 0.7, 0.0], [0.7, 0.7, 0.0]]);
    custom_mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
    custom_mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
    println!("{:#?}", custom_mesh);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(custom_mesh),
        material: material_handle.clone(),
//         transform: Transform::from_translation(mesh_location),
        ..Default::default()
    });

    // Create a reference `Cube` `shape`
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(0.8))),
        material: material_handle.clone(),
        transform: Transform::from_translation(Vec3::new(-1.5, 0.0, 0.0)),
        ..Default::default()
    });

//     println!("{:#?}", Mesh::from(shape::Cube::new(0.7)));




    // Light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(2.0, 5.0, 2.0),
        ..Default::default()
    });

    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.5, 3.0)
            .looking_at(mesh_location, Vec3::Y),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
