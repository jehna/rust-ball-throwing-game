use crate::data_channel::{ClientMessage, ServerMessage, UserId, UserMessage};
use futures_util::{SinkExt, StreamExt};
use std::io::Error;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{accept_async, tungstenite::Message};

type AllConnections = Arc<Mutex<Vec<UnboundedSender<Message>>>>;

pub fn spawn_websocket_server() -> (
    UnboundedSender<ServerMessage>,
    UnboundedReceiver<UserMessage>,
) {
    let (server_sender, server_receiver) = unbounded_channel::<ServerMessage>();
    let (user_message_sender, user_message_receiver) = unbounded_channel::<UserMessage>();
    tokio::spawn(websocket_server(server_receiver, user_message_sender));
    (server_sender, user_message_receiver)
}

async fn websocket_server(
    mut on_broadcast: UnboundedReceiver<ServerMessage>,
    on_message: UnboundedSender<UserMessage>,
) {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let all_connections = AllConnections::new(Mutex::new(Vec::new()));
    let all_connections_for_broadcast = all_connections.clone();

    tokio::spawn(async move {
        let mut id: UserId = 0;
        while let Ok((stream, _)) = listener.accept().await {
            let curr_id = id;
            id += 1;
            tokio::spawn(accept_connection(
                stream,
                curr_id,
                all_connections.clone(),
                on_message.clone(),
            ));
        }
    });

    tokio::spawn(async move {
        while let Some(message) = on_broadcast.recv().await {
            let message = bincode::serialize(&message).unwrap();
            let message = Message::binary(message);
            let connections = all_connections_for_broadcast.lock().unwrap();
            for connection in connections.iter() {
                connection.send(message.clone()).unwrap();
            }
        }
    });
}

async fn accept_connection(
    stream: TcpStream,
    user_id: UserId,
    all_connections: AllConnections,
    on_message: UnboundedSender<UserMessage>,
) -> Result<(), Error> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let (sender, mut receiver) = unbounded_channel::<Message>();
    all_connections.lock().unwrap().push(sender.clone());

    let ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    // TODO: Should come from the client
    on_message
        .send(UserMessage {
            user_id,
            message: ClientMessage::Join,
        })
        .unwrap();

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Set local player
    sender
        .send(Message::binary(
            bincode::serialize(&ServerMessage::SetLocalUserId(user_id)).unwrap(),
        ))
        .unwrap();

    tokio::spawn(async move {
        while let Some(message) = read.next().await {
            let message = message.unwrap();
            println!("Received a message from {}: {}", addr, message);
            on_message
                .send(UserMessage {
                    user_id,
                    message: bincode::deserialize(&message.into_data()).unwrap(),
                })
                .unwrap();
        }
    });

    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            write.send(message).await.unwrap();
        }
    });

    Ok(())
}
