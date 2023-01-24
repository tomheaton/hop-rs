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

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChannelState {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelToken {
    pub id: String,
    pub created_at: String,
    pub state: HashMap<String, String>,
    pub expires_at: Option<String>,
    pub is_online: bool,
    // pub is_online: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    // pub project: Project,
    pub state: HashMap<String, String>,
    // pub capabilities: i64,
    pub created_at: String,
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
