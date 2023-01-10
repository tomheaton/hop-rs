use serde::{Deserialize, Serialize};

use crate::client::APIClient;
use crate::client::DEFAULT_BASE_URL as URL;

#[derive(Debug)]
pub struct APIError;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub success: bool,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Members {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    id: String,
    name: String,
    digest: String,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {}

pub struct Projects {
    pub token: String,
}

impl Projects {
    pub fn new(
        token: &str
    ) -> Projects {
        return Projects {
            token: token.to_owned()
        };
    }

    pub async fn get_members(
        &self,
    ) -> Result<serde_json::Value, APIError> {
        println!("Getting all project members");

        return APIClient::new(
            self.token.as_str(),
        ).get("/v1/projects/@this/members").await;
    }

    pub fn get_current_member(
        &self,
    ) {
        println!("Getting current project member")
    }

    pub async fn get_tokens(
        &self,
    ) -> Result<serde_json::Value, APIError> {
        println!("Getting all project tokens");

        return APIClient::new(
            self.token.as_str(),
        ).get("/v1/projects/@this/tokens").await;
    }

    pub async fn create_token(
        &self,
        flags: i32,
    ) -> Result<serde_json::Value, APIError> {
        println!("Creating a project token with flags: {}", flags);

        return APIClient::new(
            self.token.as_str(),
        ).post(
            "/v1/projects/@this/tokens",
            &serde_json::json!(flags),
        ).await;
    }

    pub fn delete_token(
        &self,
        id: &str,
    ) {
        println!("Deleting a project token with id: {}", id);
    }

    pub async fn get_secrets(
        &self,
    ) -> Result<serde_json::Value, APIError> {
        println!("Getting all project secrets");

        return APIClient::new(
            self.token.as_str(),
        ).get("/v1/projects/@this/tokens").await;
    }

    pub async fn create_secret(
        &self,
        name: &str,
        value: String,
    ) -> Result<serde_json::Value, APIError> {
        println!("Creating a project secret with name: {} and value: {}", name, value);

        return APIClient::new(
            self.token.as_str(),
        ).put(
            format!("{}/{}", "/v1/projects/@this/secrets", name).as_str(),
            value,
        ).await;
    }

    pub fn delete_secret(
        &self,
        id: &str,
    ) {
        println!("Deleting a project secret with id: {}", id);
    }
}
