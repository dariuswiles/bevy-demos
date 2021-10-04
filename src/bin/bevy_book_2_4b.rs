/// Example from the "Creating your first plugin" section of:
/// https://bevyengine.org/learn/book/getting-started/plugins/
///
/// Displays an empty window and repeatedly runs the functions that display greetings until
/// killed. Although the result is the same as the previous example code, this code adds systems
/// via a `Plugin`.

use bevy::prelude::*;

struct Person;
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_people.system())
            .add_system(hello_world.system())
            .add_system(greet_people.system());
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}

fn hello_world() {
    println!("Hello world!");
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
