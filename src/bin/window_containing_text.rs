/// Create a window of a custom size and display some text.

use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 400.0;
const FONT_ASSET_FILENAME: &str = "fonts/FiraSans-Bold.ttf";

// This is used to prevent the program from exiting immediately
fn empty_system() {}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text::with_section(
            "Awesome test text",
            TextStyle {
                font: asset_server.load(FONT_ASSET_FILENAME),
                font_size: 100.0,
                color: Color::RED,
            },
            Default::default(),
        ),
        ..Default::default()
    });
}

fn main() {
    let wd = WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: String::from("Display test text in a window"),
        ..Default::default()
    };

    App::build()
        .insert_resource(wd)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(empty_system.system())
        .run();
}
