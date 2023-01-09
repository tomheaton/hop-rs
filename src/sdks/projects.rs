use serde::{Deserialize, Serialize};

use crate::client::APIClient;
use crate::hop::DEFAULT_BASE_URL as URL;

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
    token: String,
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

        let url = format!("{}{}", URL, "/v1/projects/@this/members");

        let client = reqwest::Client::new();
        let mut response = client
            .get(url.as_str())
            .header("Authorization", self.token.as_str())
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            println!("status: {}", response.status());
            return Err(APIError);
        }

        // let data: APIResponse<Vec<Members>> = response.json().await.unwrap();
        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {:?}", data);

        return Ok(data);
    }

    pub fn get_current_member(
        &self,
    ) {
        println!("Getting current project member")
    }

    pub fn get_tokens(
        &self,
    ) {
        println!("Getting all project tokens");
    }

    pub async fn create_token(
        &self,
        flags: i32,
    ) -> Result<serde_json::Value, APIError> {
        println!("Creating a project token with flags: {}", flags);

        // let url = format!("{}{}", URL, "/v1/projects/@this/tokens");
        let url = format!("{}{}", URL, "/v1/projects/@this/tokens");

        let client = reqwest::Client::new();
        let mut response = client
            .put(url.as_str())
            .header("Authorization", self.token.as_str())
            .header("Content-Type", "text/plain")
            .json(&serde_json::json!({
                "flags": flags,
            }))
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            println!("status: {}", response.status());
            return Err(APIError);
        }

        // let data: APIResponse<Token> = response.json().await.unwrap();
        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {:?}", data);

        return Ok(data);
    }

    pub fn delete_token(
        &self,
        id: &str,
    ) {
        println!("Deleting a project token with id: {}", id);
    }

    pub fn get_secrets(
        &self,
    ) {
        println!("Getting all project secrets");
    }

    pub async fn create_secret(
        &self,
        name: &str,
        value: String,
    ) -> Result<serde_json::Value, APIError> {
        println!("Creating a project secret with name: {} and value: {}", name, value);

        let url = format!("{}{}/{}", URL, "/v1/projects/@this/secrets", name);

        let client = reqwest::Client::new();
        let mut response = client
            .put(url.as_str())
            .header("Authorization", self.token.as_str())
            .header("Content-Type", "text/plain")
            .body(value)
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            println!("status: {}", response.status());
            return Err(APIError);
        }

        // let data: APIResponse<Secret> = response.json().await.unwrap();
        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {:?}", data);

        return Ok(data);
    }

    pub fn delete_secret(
        &self,
        id: &str,
    ) {
        println!("Deleting a project secret with id: {}", id);
    }
}
