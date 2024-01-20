mod camera;
mod debug;
mod movement;
mod spaceship;

use std::thread::spawn;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // make scene bright with bevy built-in
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.85)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.7,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
