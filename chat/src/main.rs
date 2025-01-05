use axum::{Router, Extension};
use axum::routing::post;
use firebase_auth_sdk::FireAuth;
use crate::controller::{sign_in, sign_up};

mod model;
mod controller;

#[tokio::main]
async fn main() {
    let auth_service = FireAuth::new(String::from("AIzaSyA6MS-HQZrHzL1tE4CoIAPuR3oMLCDO6t8"));

    let app = Router::new()
        .route("/signin", post(sign_in))
        .route("/signup", post(sign_up))
        .layer(Extension(auth_service));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Listening...");

    axum::serve(listener, app)
        .await
        .unwrap();
}
