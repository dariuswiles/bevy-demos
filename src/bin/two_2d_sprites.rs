// Draws two overlapping colored sprites on a 2D canvas.

use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.2, 0.2).into()),
            sprite: Sprite::new(Vec2::new(40 as f32, 60 as f32)),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        });

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 1.0, 0.2).into()),
            sprite: Sprite::new(Vec2::new(40 as f32, 60 as f32)),
            transform: Transform::from_xyz(10.0, 20.0, 0.0),
            ..Default::default()
        });

    // 2D camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}
