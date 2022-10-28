use crate::{data_channel::ServerMessage, user_movement::User};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_player_system(
    mut events: EventReader<ServerMessage>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    events
        .iter()
        .filter_map(|message| match message {
            ServerMessage::NewPlayerJoined(user_id) => Some(user_id),
            _ => None,
        })
        .for_each(|user_id| {
            // Spawn a new player
            let cube_transform = Transform::from_xyz(0.0, 0.5, 0.0);
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: cube_transform,
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(0.5, 0.5, 0.5))
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(Restitution::coefficient(0.7))
                .insert(Velocity::zero())
                .insert(User { id: *user_id });
        });
}
