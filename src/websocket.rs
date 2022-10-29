use crate::data_channel::{ClientMessage, ServerMessage};
use tokio::sync::mpsc::{Receiver, Sender};

/*#[cfg(target_arch = "wasm32")]
fn add_seven(x: i32) -> i32 {
    x + 7
}*/

#[cfg(not(target_arch = "wasm32"))]
pub async fn connect_websocket(
    mut send_queue: Receiver<ClientMessage>,
    receive_queue: Sender<ServerMessage>,
) {
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

    let (ws_stream, _) = connect_async("ws://127.0.0.1:8080")
        .await
        .expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    tokio::spawn(async move {
        while let Some(message) = send_queue.recv().await {
            let message = bincode::serialize(&message).unwrap();
            write.send(Message::binary(message)).await.unwrap();
        }
    });

    tokio::spawn(async move {
        while let Some(Ok(message)) = read.next().await {
            match message {
                Message::Binary(message) => {
                    let message = bincode::deserialize(&message).unwrap();
                    receive_queue.send(message).await.unwrap();
                }
                _ => {}
            }
        }
    });
}
