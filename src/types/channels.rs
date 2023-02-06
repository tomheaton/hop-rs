use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelType {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "unprotected")]
    Unprotected,
}

// TODO: what is this?
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChannelState {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelToken {
    /// The ID for the token
    pub id: String,
    /// State for this token
    pub state: HashMap<String, String>,
    // TODO: check this
    // /// The project this channel token is associated with
    // project_id: String,
    /// If this token is currently online (e.g. active heartbeat and connected to leap)
    pub is_online: bool,
    // TODO: check these
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    /// The ID of the channel
    pub id: String,
    // /// The project it is associated with
    // pub project: Project,
    /// State metadata
    pub state: HashMap<String, String>,
    // /// Capabilities of the channel
    // pub capabilities: i64,
    /// When this channel was created
    pub created_at: String,
    /// The type of this channel
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
}

impl Channel {
    pub fn set_state(&self) {
        println!("Setting channel state");
        panic!("not implemented!")
    }

    pub fn patch_state(&self) {
        println!("Patching channel state");
        panic!("not implemented!")
    }

    pub fn subscribe_tokens(&self) {
        println!("Subscribing tokens to channel");
        panic!("not implemented!")
    }

    pub fn subscribe_token(&self) {
        println!("Subscribing token to channel");
        panic!("not implemented!")
    }

    pub fn publish_message(&self) {
        println!("Publishing a channel message");
        panic!("not implemented!")
    }
}
