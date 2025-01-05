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
    let auth_service = FireAuth::new(String::from("AIzaSyA6MS-HQZrHzL1tE4CoIAPuR3oMLCDO6t8"));

    let (tx, _rx) = tokio::sync::broadcast::channel::<String>(100);
    let chat_state = Arc::new(ChatState { tx });

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

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// use axum::{
//     Router,
//     Extension,
//     routing::post,
//     // response::IntoResponse,
//     // http::StatusCode,
// };
// use tower_http::cors::{CorsLayer, Any};
// // use tokio::net::TcpListener;
// use std::net::SocketAddr;
// use firebase_auth_sdk::FireAuth;
// use crate::controller::{sign_in, sign_up};

// mod model;
// mod controller;

// #[tokio::main]
// async fn main() {
//     let auth_service = FireAuth::new(String::from("AIzaSyA6MS-HQZrHzL1tE4CoIAPuR3oMLCDO6t8"));

//     let cors = CorsLayer::new()
//         .allow_origin(Any)
//         .allow_methods(Any)
//         .allow_headers(Any);

//     let app = Router::new()
//         .route("/signin", post(sign_in))
//         .route("/signup", post(sign_up))
//         .layer(cors)
//         .layer(Extension(auth_service));

//     let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
//     println!("Listening on {addr}");

//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
 