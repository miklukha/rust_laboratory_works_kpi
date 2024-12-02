use warp::Filter;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

mod ws_handler;
mod auth;

#[tokio::main]
async fn main() {
    let tx = Arc::new(Mutex::new(broadcast::channel(100).0));
    let tx_ws = tx.clone();

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(auth::with_auth())
        .map(move |ws: warp::ws::Ws, claims: auth::Claims| {
            let tx = tx_ws.clone();
            ws.on_upgrade(move |websocket| ws_handler::handle_connection(websocket, tx, claims.sub))
        });

    let login_route = auth::login_filter();
    let register_route = auth::register_filter();

    let static_files = warp::fs::dir("static");

    let routes = ws_route
        .or(login_route)
        .or(register_route)
        .or(static_files)
        .recover(auth::handle_rejection);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
