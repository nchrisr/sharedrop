use crate::messages::SignalingMessage;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct Peer {
    pub id: Uuid,
    pub subnet: String,
    pub sender: mpsc::Sender<SignalingMessage>,
}
