use serde::Serialize;

use crate::types::APIError;

const BASE_URL: &str = "https://api.hop.io";

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

        // println!("{}", format!("{}{}", BASE_URL, url).as_str());
        // println!("{}", serde_json::to_string_pretty(&data).unwrap());

        let response = client
            .post(format!("{}{}", BASE_URL, url).as_str())
            .header("Authorization", self.token.as_str())
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await
            .unwrap();

        // TODO: handle returning empty response
        if response.status() == 201 {
            println!("POST Successful!");
            return Ok(serde_json::json!({}));
        }

        if response.status() != 200 {
            // println!("status: {}", response.status());
            println!("response: {}", response.text().await.unwrap());
            return Err(APIError);
        }

        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {}", serde_json::to_string_pretty(&data).unwrap());

        return Ok(data);
    }

    pub async fn put<T: Serialize>(
        &self,
        url: &str,
        data: T,
    ) -> Result<serde_json::Value, APIError> {
        let client = reqwest::Client::new();

        // println!("{}", format!("{}{}", BASE_URL, url).as_str());
        // println!("{}", serde_json::to_string_pretty(&data).unwrap());

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

    // TODO: merge this with put
    pub async fn put_none(
        &self,
        url: &str,
    ) -> Result<(), APIError> {
        let client = reqwest::Client::new();

        println!("{}", format!("{}{}", BASE_URL, url).as_str());
        // println!("{}", serde_json::to_string_pretty(&data).unwrap());

        let response = client
            .put(format!("{}{}", BASE_URL, url).as_str())
            .header("Authorization", self.token.as_str())
            .send()
            .await
            .unwrap();

        if response.status() != 201 {
            // println!("status: {}", response.status());
            println!("response: {}", response.text().await.unwrap());
            return Err(APIError);
        }

        println!("PUT Successful!");
        return Ok(());
    }

    // TODO: this is a hack, we should be able to use the same function for both
    pub async fn put_raw(
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

    pub async fn patch<T: Serialize>(
        &self,
        url: &str,
        data: T,
    ) -> Result<serde_json::Value, APIError> {
        let client = reqwest::Client::new();

        // println!("{}", format!("{}{}", BASE_URL, url).as_str());
        // println!("{}", serde_json::to_string_pretty(&data).unwrap());

        let response = client
            .patch(format!("{}{}", BASE_URL, url).as_str())
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

    pub async fn delete(
        &self,
        url: &str,
    ) -> Result<(), APIError> {
        let client = reqwest::Client::new();

        let response = client
            .delete(format!("{}{}", BASE_URL, url).as_str())
            .header("Authorization", self.token.as_str())
            .send()
            .await
            .unwrap();

        // TODO: check other functions for correct status code
        if response.status() != 204 {
            // println!("status: {}", response.status());
            println!("response: {}", response.text().await.unwrap());

            // TODO: parse json response here, because no data returned from 204 DELETE
            // let data: serde_json::Value = response.json().await.unwrap();
            // println!("response: {}", serde_json::to_string_pretty(&data).unwrap());

            return Err(APIError);
        }

        println!("DELETE Successful!");

        return Ok(());
    }

    // TODO: merge this with delete
    pub async fn delete_with_return(
        &self,
        url: &str,
    ) -> Result<serde_json::Value, APIError> {
        let client = reqwest::Client::new();

        let response = client
            .delete(format!("{}{}", BASE_URL, url).as_str())
            .header("Authorization", self.token.as_str())
            .send()
            .await
            .unwrap();

        // TODO: check other functions for correct status code
        if response.status() != 200 {
            // println!("status: {}", response.status());
            println!("response: {}", response.text().await.unwrap());

            // TODO: parse json response here, because no data returned from 204 DELETE
            // let data: serde_json::Value = response.json().await.unwrap();
            // println!("response: {}", serde_json::to_string_pretty(&data).unwrap());

            return Err(APIError);
        }

        println!("DELETE Successful!");

        let data: serde_json::Value = response.json().await.unwrap();
        println!("response: {}", serde_json::to_string_pretty(&data).unwrap());

        return Ok(data);
    }
}
