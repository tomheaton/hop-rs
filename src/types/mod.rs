use serde::{Deserialize, Serialize};

pub mod channels;
pub mod ignite;
pub mod pipe;
pub mod projects;
pub mod registry;
pub mod users;

// TODO: implement error message
#[derive(Debug, Serialize, Deserialize)]
pub struct APIError;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub success: bool,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponseOld {
    success: bool,
    message: String,
}
