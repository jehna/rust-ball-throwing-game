use crate::data_channel::{ClientDataChannelResource, ServerMessage};
use bevy::prelude::*;

pub fn server_message_handler_system(
    mut data_channel_resource: ResMut<ClientDataChannelResource>,
    mut server_messages: EventWriter<ServerMessage>,
) {
    data_channel_resource
        .receiver
        .try_recv()
        .into_iter()
        .for_each(|message| {
            server_messages.send(message);
        });
}
