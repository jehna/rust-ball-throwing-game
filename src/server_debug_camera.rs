use bevy::prelude::*;

pub fn server_debug_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 12., -2.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
