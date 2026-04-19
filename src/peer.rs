use tokio::sync::mpsc;
use uuid::Uuid;
use crate::messages::SignalingMessage;

pub struct Peer {
    pub id: Uuid,
    pub subnet: String,
    pub sender: mpsc::Sender<SignalingMessage>,
}

