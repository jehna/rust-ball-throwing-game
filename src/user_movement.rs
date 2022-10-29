use crate::current_user::CurrentUser;
use crate::data_channel::{ServerMessage, UserId};
use bevy::prelude::Component;
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct User {
    pub id: UserId,
}

pub fn user_movement(
    current_user: Res<CurrentUser>,
    mut server_events: EventReader<ServerMessage>,
    mut component: Query<(&User, &mut Transform, &mut Velocity)>,
) {
    for (user, mut transform, mut velociy) in &mut component {
        if current_user.id.is_some() && user.id == current_user.id.unwrap() {
            // Handle current user locally for now
            continue;
        }

        let move_user = server_events.iter().find(|event| match event {
            ServerMessage::NewPlayerPosition(id, ..) => *id == user.id,
            _ => false,
        });

        match move_user {
            Some(ServerMessage::NewPlayerPosition(_, position, rotation)) => {
                transform.translation = *position;
                transform.rotation = *rotation;
            }
            _ => {}
        }
    }
}
