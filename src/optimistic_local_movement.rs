use crate::{
    client_events_server_handler::apply_client_input, current_user::CurrentUser,
    data_channel::ClientMessage, user::User,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn optimistic_local_movement(
    current_user: Res<CurrentUser>,
    mut client_messages: EventReader<ClientMessage>,
    mut users: Query<(&User, &mut Transform, &mut Velocity)>,
) {
    for message in client_messages.iter() {
        match message {
            ClientMessage::Input {
                direction,
                rotation,
                jump,
            } => {
                let user = current_user
                    .id
                    .map(|id| users.iter_mut().find(|(user, ..)| user.id == id))
                    .flatten();
                let (transform, velocity) = match user {
                    Some((_, transform, velocity)) => (transform, velocity),
                    None => return,
                };
                apply_client_input(transform, velocity, *direction, *rotation, *jump);
            }
            _ => {}
        }
    }
}
