use bevy::prelude::*;
pub struct MovementPlugin;
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}
fn update_position(
    mut query: Query<(&Velocity, &mut Transform)>,
    time: Res<Time>,
    key: Res<Input<KeyCode>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        // transform.translation += velocity.value * time.delta_seconds() * 10.0;
        if key.pressed(KeyCode::X) {
            transform.rotate(Quat::from_rotation_x(time.delta_seconds() * 10.0));
        }
        if key.pressed(KeyCode::Y) {
            transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 10.0));
        }
        if key.pressed(KeyCode::Z) {
            transform.rotate(Quat::from_rotation_z(time.delta_seconds() * 10.0));
        }
        //transform.rotate(Quat::from_rotation_z(time.delta_seconds() * 20.0));
    }
}
