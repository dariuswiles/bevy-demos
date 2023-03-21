/// Create a window of a custom size and display some text.

use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 400.0;
const FONT_ASSET_FILENAME: &str = "fonts/FiraSans-Bold.ttf";

// This is used to prevent the program from exiting immediately
fn empty_system() {}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(
            Text2dBundle {
                text: Text::from_section(
                    "Awesome test text",
                    TextStyle {
                        font: asset_server.load(FONT_ASSET_FILENAME),
                        font_size: 100.0,
                        color: Color::RED,
                    },
                ),
                transform: Transform::from_xyz(-400.0, 200.0, 1.0),
                ..Default::default()
            }
        );
}

fn main() {
    let wd = WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: String::from("Display test text in a window"),
        ..Default::default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: wd,
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(empty_system)
        .run();
//     App::new()
//         .insert_resource(wd)
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup)
//         .add_system(empty_system)
//         .run();
}
