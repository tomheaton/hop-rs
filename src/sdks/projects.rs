use serde::{Deserialize, Serialize, Deserializer, de};

use crate::client::APIClient;
use crate::hop::DEFAULT_BASE_URL as URL;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
// pub struct APIResponse<T: de::DeserializeOwned> {
    pub success: bool,
    pub data: Option<T>,
    // pub data: Vec<T>,
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
    ) {
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
            return;
        }

        // let data: APIResponse<Vec<Members>> = response.json().await.unwrap();
        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {:?}", data);
    }

    pub fn get_current_member(
        &self,
    ) {
        println!("Getting current project member")
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
    ) {
        println!("Creating a project secret with name: {} and value: {}", name, value);

        let url = format!("{}{}/{}", URL, "/v1/projects/@this/secrets", name);
        println!("url: {}", url);

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
            println!("response: {:?}", response.text().await);
            return;
        }

        // let data: APIResponse<Secret> = response.json().await.unwrap();
        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {:?}", data);
    }

    pub fn delete_secret(
        &self,
        id: &str,
    ) {
        println!("Deleting a project secret with id: {}", id);
    }
}
