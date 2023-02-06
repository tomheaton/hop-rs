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

#[derive(Debug, Serialize, Deserialize)]
pub enum RoomState {
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "offline")]
    Offline,
}

// #[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    /// The ID of this stream
    pub id: String,
    /// The name of this room
    pub name: String,
    /// The unix timestamp of when this stream was created
    pub created_at: String,
    /// Protocol you can stream with
    pub ingest_protocol: IngestProtocol,
    /// Protocols that are supported by this room to the client
    pub delivery_protocols: Vec<DeliveryProtocol>,
    /// A join token to subscribe into this room
    pub join_token: String,
    /// The region that the stream url is located in
    pub ingest_region: Region,
    /// The URL that you can stream to
    pub ingest_endpoint: String,
    /// The state of the stream currently
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
