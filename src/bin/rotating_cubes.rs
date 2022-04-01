/// Create several rotating cubes, a light source, and a camera. The rotation is controlled by
/// adding the `RotatingEntity` component to the entities to be rotated.

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

    let cube_locations = [
        Vec3::new(0.0, 0.0, -5.0),
        Vec3::new(-4.0, 0.0, -9.0),
        Vec3::new(3.0, 0.0, -13.0),
        Vec3::new(-1.0, 0.0, -17.0),
    ];

    for location in cube_locations.iter() {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
                material: material_handle.clone(),
                transform: Transform::from_translation(*location),
                ..Default::default()
            })
            .insert(RotatingEntity);
    }

    // Light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(2.0, 5.0, 2.0),
        ..Default::default()
    });

    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    });
}


fn rotate_entities(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingEntity>>) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(time.seconds_since_startup() as f32 / 2.0);
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_entities)
        .run();
}
