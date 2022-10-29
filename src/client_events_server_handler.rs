use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    data_channel::{ClientMessage, ServerMessage, UserMessage},
    user::User,
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
            let user = component.iter_mut().find(|(user, ..)| user.id == user_id);

            match message {
                ClientMessage::Join => {
                    server_messages.send(ServerMessage::NewPlayerJoined(user_id))
                }
                ClientMessage::Input {
                    direction,
                    rotation,
                    jump,
                } => {
                    let (transform, velocity) = match user {
                        Some((_, transform, velocity)) => (transform, velocity),
                        None => return,
                    };
                    apply_client_input(transform, velocity, direction, rotation, jump);
                }
            }
        });
}

pub fn server_events_broadcaster(
    server_broadcaster: ResMut<UnboundedSender<ServerMessage>>,
    mut server_messages: EventReader<ServerMessage>,
) {
    for message in server_messages.iter() {
        server_broadcaster.send(message.clone()).unwrap();
    }
}

pub fn apply_client_input(
    transform: Mut<Transform>,
    mut velocity: Mut<Velocity>,
    direction: Vec2,
    rotation: f32,
    jump: bool,
) {
    velocity.angvel.y = rotation * -1.;
    let movement = transform.forward() * direction.x + transform.right() * direction.y;
    velocity.linvel += movement * 0.8;
    if jump {
        velocity.linvel.y = 3.0;
    }
}
