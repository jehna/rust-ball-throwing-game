use std::sync::{Arc, Mutex};

use bevy::prelude::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::websocket::connect_websocket;

pub type UserId = u16;

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    NewPlayerPosition(UserId, Vec3),
    NewPlayerJoined(UserId),
    SetLocalUserId(UserId),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Move(Vec2),
    Jump,
}

pub struct ClientDataChannelResource {
    pub sender: Sender<ClientMessage>,
    pub receiver: Receiver<ServerMessage>,
}

pub fn create_data_channel() -> ClientDataChannelResource {
    let (client_sender, client_receiver) = channel::<ClientMessage>(1024);
    let (server_sender, server_receiver) = channel::<ServerMessage>(1024);

    let data_channel = ClientDataChannelResource {
        sender: client_sender,
        receiver: server_receiver,
    };

    tokio::spawn(async move {
        connect_websocket(client_receiver, server_sender).await;
    });

    data_channel
}
