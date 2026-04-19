use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="type", rename_all="snake_case")]
pub enum SignalingMessage{
    PeerJoined {peer_id: Uuid},
    PeerLeft {peer_id: Uuid},
    Offer {from: Uuid, to: Uuid, sdp: String},
    Answer {from: Uuid, to: Uuid, sdp: String},
    IceCandidate {from: Uuid, to: Uuid, candidate: String}
}
