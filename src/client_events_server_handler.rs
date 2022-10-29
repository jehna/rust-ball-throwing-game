use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{
    data_channel::{ClientMessage, ServerMessage, UserMessage},
    user_movement::User,
};

pub fn client_events_server_handler(
    mut client_events: ResMut<UnboundedReceiver<UserMessage>>,
    mut server_messages: EventWriter<ServerMessage>,
    mut component: Query<(&User, &mut Transform, &mut Velocity)>,
) {
    client_events
        .try_recv()
        .into_iter()
        .for_each(|user_message| {
            let UserMessage { user_id, message } = user_message;
            match message {
                ClientMessage::Join => {
                    server_messages.send(ServerMessage::NewPlayerJoined(user_id))
                }
                ClientMessage::Move(position, rotation) => server_messages.send(
                    ServerMessage::NewPlayerPosition(user_id, position, rotation),
                ),
            }
        });
}

pub fn server_events_broadcaster(
    server_broadcaster: ResMut<UnboundedSender<ServerMessage>>,
    mut server_messages: EventReader<ServerMessage>,
) {
    for message in server_messages.iter() {
        server_broadcaster.send(*message).unwrap();
    }
}
