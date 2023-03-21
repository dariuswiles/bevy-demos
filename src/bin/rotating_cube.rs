/// Create two cubes, a light source, and a camera, and position them such that the main cube is
/// visible from the camera. The main cube rotates, the other doesn't. This is controlled by
/// adding the `RotatingEntity` component to the main cube. Theoretically, the `rotate_entities`
/// system should rotate all types of entities that have this component.

use bevy::prelude::*;

#[derive(Component)]
struct RotatingEntity; // Component to indicate entity should be rotated

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

    // Create a mesh from a `Cube` `shape`
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0)),
            ..Default::default()
        })
        .insert(RotatingEntity);

    // Another cube, this time without `RotatingEntity`.
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: material_handle.clone(),
        transform: Transform::from_translation(Vec3::new(-4.0, 0.0, -9.0)),
        ..Default::default()
    });

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(2.0, 5.0, 2.0),
        ..Default::default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    });
}


fn rotate_entities(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingEntity>>) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(time.elapsed_seconds() as f32 / 2.0);
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_entities)
        .run();
}
