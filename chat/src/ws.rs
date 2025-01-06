use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::Extension,
    response::IntoResponse,
    // routing::get,
};
use futures_util::{SinkExt, StreamExt};
use std::{sync::Arc};
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use serde_json;

// зберігання стану чату
#[derive(Clone)]
pub struct ChatState {
    pub tx: broadcast::Sender<String>,
}
// вхідні повідомлення 
#[derive(Deserialize)]
struct IncomingMessage {
    userId: String,
    email: String,
    message: String,
}
// вихідні повідомлення
#[derive(Serialize)]
struct OutgoingMessage {
    email: String,   
    message: String,
}

// обробник WebSocket з'єднання
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<ChatState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<ChatState>) {
    // розділення WebSocket на відправника та приймача
    let (mut ws_sender, mut ws_receiver) = socket.split();
    
    // підписка на канал трансляції повідомлень
    let mut rx = state.tx.subscribe();

    // відправка повідомлень клієнту
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if ws_sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // обробка вхідних повідомлень від клієнта
    while let Some(Ok(Message::Text(text))) = ws_receiver.next().await {
        match serde_json::from_str::<IncomingMessage>(&text) {
            Ok(in_msg) => {
                let out = OutgoingMessage {
                    email: in_msg.email, 
                    message: in_msg.message,
                };

                if let Ok(json_out) = serde_json::to_string(&out) {
                    let _ = state.tx.send(json_out);
                }
            }
            Err(e) => {
                eprintln!("Failed to parse incoming WS message as JSON: {:?}", e);
            }
        }
    }

    send_task.abort();
}
