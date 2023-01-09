use serde::{Deserialize, Serialize};

const URL: &str = "https://tomheaton.dev/api/hello";

#[derive(Debug, Deserialize)]
struct APIResponse {
    success: bool,
    message: String,
}

#[derive(Clone, Copy)]
pub struct APIClient {
    options: i32,
    agent: i32,

    pub auth_type: i32,
    pub url: i32,
}

impl APIClient {
    pub fn new() -> APIClient {
        return APIClient {
            options: 1,
            agent: 1,
            auth_type: 1,
            url: 1,
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
        let mut response = reqwest::get(url).await.unwrap();

        if response.status() != 200 {
            println!("status: {}", response.status());
            return;
        }

        let data_struct: APIResponse = response.json().await.unwrap();
        println!("response: {:?}", data_struct);
    }
}
