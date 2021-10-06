/// Create a cube, a light source, and a camera, and position them such that the lit cube is
/// visible from the camera.

use bevy::prelude::*;

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
    let cube_location = Vec3::new(0.0, 0.0, 4.0);

    // Create a mesh from a `Cube` `shape`
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: material_handle.clone(),
        transform: Transform::from_translation(cube_location),
        ..Default::default()
    });

    // Light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(2.0, 5.0, 2.0),
        ..Default::default()
    });

    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(1.0, 2.0, 0.0)
            .looking_at(cube_location, Vec3::Y),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}
