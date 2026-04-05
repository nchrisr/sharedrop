use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::{Router, response::IntoResponse, routing::get};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use uuid::Uuid;

const HOST: &str = "0.0.0.0";
const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    let address = format!("{}:{}", HOST, PORT);
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
    let router = Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new("static"));
    let listener = TcpListener::bind(&address).await.unwrap();
    tracing::info!("Server running on {}", &address);
    axum::serve(listener, router).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket: WebSocket| async move {
        // socket is the live WebSocket connection, handle it here.
        let peer_id = Uuid::new_v4();
        tracing::info!("ID: {} assigned to new peer", peer_id);
        tracing::info!("Peer {} connected", peer_id);
        tracing::info!("Peer {} disconnected", peer_id);
    })
}
