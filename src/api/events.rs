use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::Response;
use tokio::sync::broadcast;

use crate::live::LiveEvent;
use crate::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state.live_tx.subscribe()))
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<LiveEvent>) {
    tracing::info!("WebSocket client connected");

    loop {
        tokio::select! {
            event = rx.recv() => {
                match event {
                    Ok(live_event) => {
                        if let Ok(json) = serde_json::to_string(&live_event) {
                            if socket.send(Message::Text(json.into())).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!("WebSocket client lagged by {} messages", n);
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(data))) => {
                        if socket.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    _ => {} // Ignore other messages
                }
            }
        }
    }

    tracing::info!("WebSocket client disconnected");
}
