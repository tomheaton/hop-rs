use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    flags: i64,
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    id: String,
    joined_at: String,
    mfa_enabled: bool,
    name: String,
    username: String,
    role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    id: String,
    name: String,
    digest: String,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {}
