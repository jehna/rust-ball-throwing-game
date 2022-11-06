use std::vec;

use crate::{data_channel::ServerMessage, env::Environment, user::User};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_player_system(
    env: Res<Environment>,
    mut events: EventReader<ServerMessage>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    players: Query<&User>,
) {
    events
        .iter()
        .filter_map(|message| match message {
            ServerMessage::Snapshot { users } => Some(users.iter().map(|(id, _)| id).collect()),
            ServerMessage::SetLocalUserId(id) => Some(vec![id]),
            ServerMessage::NewPlayerJoined(id) => Some(vec![id]),
        })
        .flatten()
        .filter_map(|id| {
            if players.iter().find(|user| user.id == *id).is_none() {
                Some(id)
            } else {
                None
            }
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
                .insert(if env.is_server() {
                    RigidBody::Dynamic
                } else {
                    RigidBody::KinematicPositionBased
                })
                .insert(Collider::cuboid(0.5, 0.5, 0.5))
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(Restitution::coefficient(0.1))
                .insert(Velocity::zero())
                .insert(Damping {
                    linear_damping: 2.,
                    angular_damping: 5.,
                })
                .insert(User { id: *user_id });
        });
}
