use serde::{Deserialize, Serialize};

pub mod channels;
pub mod ignite;
pub mod pipe;
pub mod projects;
pub mod registry;
pub mod users;

#[derive(Debug)]
pub struct APIError;

#[derive(Deserialize, Debug)]
pub struct APIResponse<T> {
    pub success: bool,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
pub struct APIResponseOld {
    success: bool,
    message: String,
}
