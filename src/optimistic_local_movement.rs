use crate::{
    client_events_server_handler::apply_client_input, current_user::CurrentUser,
    data_channel::ClientMessage, user::User, user_input::ClientMessagesQueue,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn optimistic_local_movement(
    current_user: Res<CurrentUser>,
    client_messages: Res<ClientMessagesQueue>,
    mut users: Query<(&User, &mut Transform, &mut Velocity)>,
) {
    let Some(ClientMessage::Input {
        direction,
        rotation,
        jump,
    }) = client_messages.last() else {
        return;
    };

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
