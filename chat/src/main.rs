use axum::{
    Router,
    Extension,
    routing::{post, get}
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use firebase_auth_sdk::FireAuth;
use std::sync::Arc;

mod model;
mod controller;
mod ws;

use crate::controller::{sign_in, sign_up};
use crate::ws::{ws_handler, ChatState};

#[tokio::main]
async fn main() {
    // підключення до firebase
    let auth_service = FireAuth::new(String::from("AIzaSyA6MS-HQZrHzL1tE4CoIAPuR3oMLCDO6t8"));

    // створення каналу для передачі повідомлень між потоками
    let (tx, _rx) = tokio::sync::broadcast::channel::<String>(100);
    let chat_state = Arc::new(ChatState { tx });

    // налаштування CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/signin", post(sign_in))
        .route("/signup", post(sign_up))
        .route("/ws", get(ws_handler))
        .layer(cors)
        .layer(Extension(auth_service))
        .layer(Extension(chat_state));

    // адреса сервера
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {addr}");

    // запуск сервера
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}