// Draws two overlapping colored sprites on a 2D canvas.

use bevy::prelude::*;

fn setup(
    mut commands: Commands,
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(40.0, 60.0)),
                color: Color::rgb(1.0, 0.2, 0.2).into(),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(40.0, 60.0)),
                color: Color::rgb(0.2, 1.0, 0.2).into(),
                ..Default::default()
            },
            transform: Transform::from_xyz(10.0, 20.0, 0.0),
            ..Default::default()
        });

    // 2D camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}
