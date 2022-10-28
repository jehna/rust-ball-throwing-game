use futures_util::{SinkExt, StreamExt};
use rust_ball_throwing_multipleyer_game::data_channel::ServerMessage;
use std::io::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) -> Result<(), Error> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    // Sleep for 2 seconds, then send a message.
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    write.send(Message::Ping(vec![1])).await.unwrap();

    let messages = vec![
        &ServerMessage::NewPlayerJoined(123),
        &ServerMessage::SetLocalUserId(123),
    ];

    for message in messages.iter() {
        let encoded = bincode::serialize(message).unwrap();
        write.send(Message::Binary(encoded)).await.unwrap();
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    write
        .send(Message::Binary(
            bincode::serialize(&ServerMessage::NewPlayerJoined(111)).unwrap(),
        ))
        .await
        .unwrap();

    // Keep socket open until the client closes it.
    while let Some(message) = read.next().await {
        let message = message.unwrap();
        println!("Received a message from {}: {}", addr, message);
    }

    Ok(())
}
