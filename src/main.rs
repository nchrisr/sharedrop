use axum::{
    extract::{
        connect_info::ConnectInfo,
        State,
        ws::{WebSocket, WebSocketUpgrade}
    }};
use axum::{Router, response::IntoResponse, routing::get};
use std::{collections::HashMap, net::{IpAddr, SocketAddr}, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::services::ServeDir;
use uuid::Uuid;

type PeerMap = Arc<Mutex<HashMap<String, HashMap<Uuid, ()>>>>;

const HOST: &str = "0.0.0.0";
const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    let peers: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    let router = Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(peers);

    let address = format!("{}:{}", HOST, PORT);
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let listener = TcpListener::bind(&address).await.unwrap();
    tracing::info!("Server running on {}", &address);
    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .await.unwrap();
}

async fn ws_handler(
    State(peers): State<PeerMap>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ws: WebSocketUpgrade
) -> impl IntoResponse {
    ws.on_upgrade( move |mut socket: WebSocket| async move {
        // socket is the live WebSocket connection, handle it here.
        let Some(subnet) = subnet_key(addr) else {
            tracing::warn!("Rejected non-IPV4 connection from {}", addr);
            return;
        };

        let peer_id = Uuid::new_v4();
        tracing::info!("ID: {} assigned to new peer", peer_id);
        tracing::info!("Peer {} connected from subnet {}", peer_id, subnet);

        {
            let mut map = peers.lock().await;
            if !map.contains_key(&subnet) {
                map.insert(subnet.clone(), HashMap::new());
            }
            map.get_mut(&subnet).unwrap().insert(peer_id, ());

            for key in map.keys() {
                tracing::info!(
                    "{} Active peers on subnet {}",
                    map.get(key).map(|m| m.len()).unwrap_or(0),
                    key
                );
            }

            tracing::info!("Total active subnets: {}", map.len());

        }

        while let Some(msg) = socket.recv().await {
            // handle message or just break out on error
            if msg.is_err() {
                break;
            }
        }

        {
            let mut map = peers.lock().await;
            if let Some(subnet_map) = map.get_mut(&subnet) {
                subnet_map.remove(&peer_id);
                if subnet_map.is_empty() {
                    map.remove(&subnet);
                }
            }
        }

        tracing::info!("Peer {} disconnected", peer_id);

    })
}

fn subnet_key(addr: SocketAddr) -> Option<String> {
    match addr.ip() {
        IpAddr::V4(ip) => {
            let octets = ip.octets(); // get an array of the parts of the IP address.
            format!("{}_{}_{}", octets[0], octets[1], octets[2]).into()
            // build the key from the first 3 octets
        },
        IpAddr::V6(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subnet_key_ipv4(){
        let _addr: SocketAddr = "198.168.1.5:3000".parse().unwrap();
        assert_eq!(subnet_key(_addr), Some("198_168_1".to_string()));
    }

    #[test]
    fn test_subnet_key_ipv6(){
        // [::1] is the ipv6 equivalent of localhost
        let _addr: SocketAddr = "[::1]:3000".parse().unwrap();
        assert_eq!(subnet_key(_addr), None);
    }
}
