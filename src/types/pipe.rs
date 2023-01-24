use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum IngestProtocol {
    #[serde(rename = "rtmp")]
    RTMP,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeliveryProtocol {
    #[serde(rename = "webrtc")]
    WebRTC,
    // TODO: check with hop
    // #[serde(rename = "hls")]
    // HLS,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "us-east-1")]
    USEast1,
}

// TODO: rename enums to PascalCase
#[derive(Debug, Serialize, Deserialize)]
pub enum RoomState {
    #[serde(rename = "live")]
    LIVE,
    #[serde(rename = "offline")]
    OFFLINE,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub ingest_protocol: IngestProtocol,
    pub delivery_protocols: Vec<DeliveryProtocol>,
    pub join_token: String,
    pub ingest_region: Region,
    pub ingest_endpoint: String,
    pub state: RoomState,
}

impl Room {
    pub fn delete(&self) {
        println!("Deleting a room");
        panic!("not implemented!");
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomOptions {
    pub delivery_protocols: Vec<DeliveryProtocol>,
    pub ephemeral: bool,
    pub ingest_protocol: IngestProtocol,
    pub hls_config: Option<HLSConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HLSConfig {
    pub wcl_delay: i64,
    pub artificial_id: i64,
    pub max_playout_bitrate_preset: String,
}
