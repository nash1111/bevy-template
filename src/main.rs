mod debug;
mod movement;
mod spaceship;

use std::thread::spawn;

use bevy::prelude::*;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
