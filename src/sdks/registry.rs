use crate::client::APIClient;
use crate::types::APIError;
use crate::types::registry::Image;

pub struct Registry {
    pub token: String,
    pub client: APIClient,
}

impl Registry {
    pub fn new(
        token: &str,
    ) -> Registry {
        return Registry {
            token: token.to_owned(),
            client: APIClient::new(token),
        };
    }

    // Images:

    pub async fn get_images(
        &self
        // TODO: add image return here
        // ) -> Result<Vec<Image>, APIError> {
    ) -> Result<Vec<String>, APIError> {
        let response = self.client.get(
            "/v1/registry/images"
        ).await.unwrap();

        let images = response["data"]["images"].clone();

        return Ok(serde_json::from_value(images).unwrap());
    }

    pub async fn delete_image(
        &self,
        image: &str,
    ) -> Result<(), APIError> {
        self.client.delete(
            format!("/v1/registry/images/{}", image).as_str(),
        ).await.unwrap();

        return Ok(());
    }

    pub async fn get_manifest(
        &self,
        image: &str,
    ) -> Result<Vec<Image>, APIError> {
        let response = self.client.get(
            format!("/v1/registry/{}/manifest", image).as_str(),
        ).await.unwrap();

        let manifest = response["data"]["image"].clone();

        return Ok(serde_json::from_value(manifest).unwrap());
    }
}
