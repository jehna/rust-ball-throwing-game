use futures_util::{SinkExt, StreamExt};
use rust_ball_throwing_multipleyer_game::data_channel::{ServerMessage, UserId};
use std::io::Error;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio_tungstenite::{accept_async, tungstenite::Message};

type AllConnections = Arc<Mutex<Vec<UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let all_connections = AllConnections::new(Mutex::new(Vec::new()));

    let mut id: UserId = 0;
    while let Ok((stream, _)) = listener.accept().await {
        let curr_id = id;
        id += 1;
        tokio::spawn(accept_connection(stream, curr_id, all_connections.clone()));
    }

    Ok(())
}

async fn accept_connection(
    stream: TcpStream,
    user_id: UserId,
    all_connections: AllConnections,
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

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Broadcast new player
    for connection in all_connections.lock().unwrap().iter() {
        let message = bincode::serialize(&ServerMessage::NewPlayerJoined(user_id)).unwrap();
        connection.send(Message::binary(message)).unwrap();
    }

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
        }
    });

    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            write.send(message).await.unwrap();
        }
    });

    Ok(())
}
