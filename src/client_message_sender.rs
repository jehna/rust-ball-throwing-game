use bevy::prelude::*;

use crate::{
    data_channel::{ClientDataChannelResource, ClientMessage},
    user_input::ClientMessagesQueue,
};

pub struct ClientMessageSendTimer {
    timer: Timer,
}

impl Default for ClientMessageSendTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1. / 33., true),
        }
    }
}

pub fn client_message_sender(
    time: Res<Time>,
    data_channel: Res<ClientDataChannelResource>,
    mut send_timer: ResMut<ClientMessageSendTimer>,
    mut client_message_queue: ResMut<ClientMessagesQueue>,
) {
    send_timer.timer.tick(time.delta());
    if send_timer.timer.just_finished() && !client_message_queue.is_empty() {
        let message = combine_pending(client_message_queue.iter());

        data_channel.sender.try_send(message).unwrap();
        client_message_queue.clear();
    }
}

fn combine_pending<'a>(messages: impl Iterator<Item = &'a ClientMessage>) -> ClientMessage {
    let mut direction = Vec2::ZERO;
    let mut rotation = 0.;
    let mut jump = false;

    for message in messages {
        match message {
            ClientMessage::Input {
                direction: d,
                rotation: r,
                jump: j,
            } => {
                direction = d.clone();
                rotation += r;
                jump |= j;
            }
            _ => {}
        }
    }

    ClientMessage::Input {
        direction,
        rotation,
        jump,
    }
}
