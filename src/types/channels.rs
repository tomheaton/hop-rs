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
pub struct Channel {
    pub id: String,
    pub project: Project,
    pub state: HashMap<String, String>,
    pub capabilities: i64,
    pub created_at: String,
    pub channel_type: ChannelType,
}
