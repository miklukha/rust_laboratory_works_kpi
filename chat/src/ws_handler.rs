use warp::ws::{WebSocket, Message};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use futures_util::{StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct ClientMessage {
    username: String,
    message: String,
}

pub async fn handle_connection(ws: WebSocket, tx: Arc<Mutex<broadcast::Sender<String>>>, username: String) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let mut rx = tx.lock().unwrap().subscribe();

    // let username_clone = username.clone();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if ws_sender.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(message) => {
                if let Ok(text) = message.to_str() {
                    let client_msg = ClientMessage {
                        username: username.clone(),
                        message: text.to_string(),
                    };
                    let msg_json = serde_json::to_string(&client_msg).unwrap();
                    tx.lock().unwrap().send(msg_json).expect("Failed to broadcast message");
                }
            },
            Err(_) => break,
        }
    }
}
