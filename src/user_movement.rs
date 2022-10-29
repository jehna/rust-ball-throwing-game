use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

use crate::{data_channel::ServerMessage, user::User};

pub fn user_movement(
    mut server_messages: EventReader<ServerMessage>,
    mut users: Query<(&User, &mut Transform, &mut Velocity)>,
) {
    for message in server_messages.iter() {
        match message {
            ServerMessage::Snapshot { users: snapshot } => {
                for (user, mut transform, mut velocity) in users.iter_mut() {
                    if let Some(snapshot) = snapshot.get(&user.id) {
                        transform.translation = snapshot.position;
                        transform.rotation = snapshot.rotation;
                        velocity.linvel = snapshot.linevel;
                        velocity.angvel = snapshot.angvel;
                    }
                }
            }
            _ => {}
        }
    }
}
