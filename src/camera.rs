use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, rotate_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}

fn rotate_camera(mut commands: Commands, time: Res<Time>, mut query: Query<&mut Transform>) {
    for mut transform in query.iter_mut() {
        //transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
        //transform.rotate(Quat::from_rotation_x(time.delta_seconds()));
        //transform.rotate(Quat::from_rotation_z(time.delta_seconds() * 0.5));
    }
}
