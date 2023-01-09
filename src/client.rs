use serde::{Deserialize, Serialize};

use crate::hop::DEFAULT_BASE_URL as URL;

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
        base_url: &str,
        token: &str,
    ) -> APIClient {
        return APIClient {
            token: token.to_owned(),
        };
    }

    pub async fn get(
        &self,
    ) {
        // return self.request(URL).await;
        self.request(URL).await;
    }

    pub async fn post(
        &self
    ) {
        // return self.request(URL).await;
        self.request(URL).await;
    }

    pub async fn put(
        &self
    ) {
        // return self.request(URL).await;
        self.request(URL).await;
    }

    pub async fn patch(
        &self
    ) {
        // return self.request(URL).await;
        self.request(URL).await;
    }

    pub async fn delete(
        &self
    ) {
        // return self.request(URL).await;
        self.request(URL).await;
    }

    pub fn raw(
        &self
    ) {
        return self.execute_request();
    }

    fn execute_request(
        &self
    ) {}

    async fn request(
        &self,
        url: &str,
    ) {
        let mut response = reqwest::get(URL).await.unwrap();

        if response.status() != 200 {
            println!("status: {}", response.status());
            return;
        }

        let data: APIResponse = response.json().await.unwrap();
        println!("response: {:?}", data);
    }
}
