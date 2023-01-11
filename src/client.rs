use serde::{Deserialize, Serialize};

use crate::sdks::projects::APIError;

// pub const DEFAULT_BASE_URL: &str = "https://tomheaton.dev/api/hello";
pub const DEFAULT_BASE_URL: &str = "https://api.hop.io";
pub const BASE_URL: &str = "https://api.hop.io";

#[derive(Debug, Deserialize)]
struct APIResponse {
    success: bool,
    message: String,
}

#[derive(Clone)]
pub struct APIClient {
    pub token: String,
}

impl APIClient {
    pub fn new(
        token: &str,
    ) -> APIClient {
        return APIClient {
            token: token.to_string(),
        };
    }

    pub async fn get(
        &self,
        url: &str,
    ) -> Result<serde_json::Value, APIError> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}{}", BASE_URL, url).as_str())
            .header("Authorization", self.token.as_str())
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            // println!("status: {}", response.status());
            println!("response: {}", response.text().await.unwrap());
            return Err(APIError);
        }

        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {}", serde_json::to_string_pretty(&data).unwrap());

        return Ok(data);
    }

    pub async fn post<T: Serialize>(
        &self,
        url: &str,
        data: T,
    ) -> Result<serde_json::Value, APIError> {
        let client = reqwest::Client::new();

        println!("{}", format!("{}{}", BASE_URL, url).as_str());
        println!("{}", serde_json::to_string_pretty(&data).unwrap());

        let response = client
            .put(format!("{}{}", BASE_URL, url).as_str())
            .header("Authorization", self.token.as_str())
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            // println!("status: {}", response.status());
            println!("response: {}", response.text().await.unwrap());
            return Err(APIError);
        }

        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {}", serde_json::to_string_pretty(&data).unwrap());

        return Ok(data);
    }

    pub async fn put(
        &self,
        url: &str,
        data: String,
    ) -> Result<serde_json::Value, APIError> {
        let client = reqwest::Client::new();

        let response = client
            .put(format!("{}{}", BASE_URL, url).as_str())
            .header("Authorization", self.token.as_str())
            .header("Content-Type", "text/plain")
            .body(data)
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            // println!("status: {}", response.status());
            println!("response: {}", response.text().await.unwrap());
            return Err(APIError);
        }

        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {}", serde_json::to_string_pretty(&data).unwrap());

        return Ok(data);
    }

    pub async fn patch(
        &self,
        url: &str,
    ) -> Result<serde_json::Value, APIError> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}{}", BASE_URL, url).as_str())
            .header("Authorization", self.token.as_str())
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            // println!("status: {}", response.status());
            println!("response: {}", response.text().await.unwrap());
            return Err(APIError);
        }

        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {}", serde_json::to_string_pretty(&data).unwrap());

        return Ok(data);
    }

    pub async fn delete(
        &self,
        url: &str,
    ) -> Result<serde_json::Value, APIError> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}{}", BASE_URL, url).as_str())
            .header("Authorization", self.token.as_str())
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            // println!("status: {}", response.status());
            println!("response: {}", response.text().await.unwrap());
            return Err(APIError);
        }

        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {}", serde_json::to_string_pretty(&data).unwrap());

        return Ok(data);
    }
}
