use serde::Deserialize;

use crate::client::APIClient;
use crate::sdks::projects::APIError;

#[derive(Deserialize, Debug)]
pub struct Image {}

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub tag: String,
    pub digest: Digest,
}

#[derive(Deserialize, Debug)]
pub struct Digest {
    pub digest: String,
    pub size: i64,
    pub uploaded: String,
}

pub struct Registry {
    pub token: String,
}

impl Registry {
    pub fn new(
        token: &str,
    ) -> Registry {
        return Registry {
            token: token.to_owned(),
        };
    }

    pub async fn get_images(
        &self
        // ) -> Result<Vec<Image>, APIError> {
    ) -> Result<Vec<String>, APIError> {
        println!("Getting all images");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            "/v1/registry/images"
        ).await.unwrap();

        let images = response["data"]["images"].clone();

        return Ok(serde_json::from_value(images).unwrap());
    }

    pub async fn get_manifest(
        &self,
        image: &str,
    ) -> Result<Vec<Image>, APIError> {
        println!("Getting image manifest");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            format!("/v1/registry/{}/manifest", image).as_str(),
        ).await.unwrap();

        let manifest = response["data"]["image"].clone();

        return Ok(serde_json::from_value(manifest).unwrap());
    }

    pub async fn delete_image(
        &self,
        image: &str,
    ) -> Result<(), APIError> {
        println!("Deleting an image with image: {}", image);

        APIClient::new(
            self.token.as_str(),
        ).delete(
            format!("/v1/registry/images/{}", image).as_str(),
        ).await.unwrap();

        return Ok(());
    }
}
