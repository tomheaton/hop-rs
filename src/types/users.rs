use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub username: String,
    pub email_verified: bool,
    pub mfa_enabled: bool,
    pub totp_enabled: bool,
    pub webauthn_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pat {
    pub id: String,
    pub pat: String,
    pub created_at: String,
    pub name: Option<String>,
}
