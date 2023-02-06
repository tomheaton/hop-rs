use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// The ID of the user
    pub id: String,
    /// The name of the user.
    pub name: String,
    /// A unique username for the user
    pub username: String,
    /// The email of the user
    pub email: String,
    // TODO: check these (only returned from API)
    pub email_verified: bool,
    pub mfa_enabled: bool,
    pub totp_enabled: bool,
    pub webauthn_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pat {
    /// The ID of the pat
    pub id: String,
    /// The name of the pat
    pub name: Option<String>,
    /// The pat token
    pub pat: String,
    /// The date the pat was created
    pub created_at: String,
}
