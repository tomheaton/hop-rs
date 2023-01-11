use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Role {
    flags: i64,
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Member {
    id: String,
    joined_at: String,
    mfa_enabled: bool,
    name: String,
    username: String,
    role: Role,
}

#[derive(Deserialize, Debug)]
pub struct Secret {
    id: String,
    name: String,
    digest: String,
    created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct Token {}
