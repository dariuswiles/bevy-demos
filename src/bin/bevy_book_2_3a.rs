/// Example from the "Your First System" section of:
/// https://bevyengine.org/learn/book/getting-started/ecs/

use bevy::prelude::*;

fn hello_world() {
    println!("Hello world!");
}

fn main() {
    App::new()
        .add_system(hello_world.system())
        .run();
}
