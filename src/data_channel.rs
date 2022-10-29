use bevy::{
    prelude::{Quat, Vec2, Vec3},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{user::UserId, websocket::connect_websocket};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerSnapshot {
    pub position: Vec3,
    pub rotation: Quat,
    pub linevel: Vec3,
    pub angvel: Vec3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    NewPlayerJoined(UserId),
    SetLocalUserId(UserId),
    Snapshot {
        users: HashMap<UserId, PlayerSnapshot>,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ClientMessage {
    Join,
    Input {
        direction: Vec2,
        rotation: f32,
        jump: bool,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct UserMessage {
    pub user_id: UserId,
    pub message: ClientMessage,
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
