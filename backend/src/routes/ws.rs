use axum::{
    Router,
    routing::get,
    extract::WebSocketUpgrade,
    response::IntoResponse,
};
use axum::extract::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt};
use tokio::sync::broadcast;
use once_cell::sync::Lazy;

// Global broadcast channel (128 buffer)
static CHANNEL: Lazy<broadcast::Sender<String>> =
    Lazy::new(|| broadcast::channel(128).0);

pub fn router() -> Router {
    Router::new().route("/ws/prices", get(ws_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut rx = CHANNEL.subscribe();

    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}

// Called by OracleManager
pub fn broadcast_price(json: &str) {
    let _ = CHANNEL.send(json.to_string());
}
