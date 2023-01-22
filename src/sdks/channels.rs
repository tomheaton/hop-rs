use std::collections::HashMap;

use crate::client::APIClient;
use crate::types::APIError;
use crate::types::channels::Channel;

pub struct Channels {
    pub token: String,
}

impl Channels {
    pub fn new(
        token: &str,
    ) -> Channels {
        return Channels {
            token: token.to_owned(),
        };
    }

    // Channels:

    pub async fn get_channels(
        &self
    ) -> Result<Vec<Channel>, APIError> {
        println!("Getting all channels");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            "/v1/channels"
        ).await.unwrap();

        let channels = response["data"]["channels"].clone();

        return Ok(serde_json::from_value(channels).unwrap());
    }

    pub async fn get_channel(
        &self,
        channel_id: &str,
    ) -> Result<Channel, APIError> {
        println!("Getting a channel");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            format!("/v1/channels/{}", channel_id).as_str()
        ).await.unwrap();

        let channel = response["data"]["channel"].clone();

        return Ok(serde_json::from_value(channel).unwrap());
    }

    pub async fn create_channel(
        &self
    ) -> () {
        println!("Creating a channel");
        panic!("not implemented!");
    }

    pub async fn delete_channel(
        &self
    ) -> () {
        println!("Deleting a channel");
        panic!("not implemented!");
    }

    pub async fn get_stats(
        &self,
        channel_id: &str,
        // TODO: improve this (ask hop about this)
    ) -> Result<HashMap<String, i64>, APIError> {
        println!("Getting a channel's stats");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            format!("/v1/channels/{}/stats", channel_id).as_str()
        ).await.unwrap();

        let stats = response["data"]["stats"].clone();

        return Ok(serde_json::from_value(stats).unwrap());
    }

    pub async fn subscribe_tokens(
        &self
    ) -> () {
        println!("Getting channel subscription tokens");
        panic!("not implemented!");
    }

    pub async fn subscribe_token(
        &self
    ) -> () {
        println!("Getting channel subscription token");
        panic!("not implemented!");
    }

    pub async fn get_tokens(
        &self
    ) -> () {
        println!("Getting all channel tokens");
        panic!("not implemented!");
    }

    pub async fn set_state(
        &self
    ) -> () {
        println!("Setting channel state");
        panic!("not implemented!");
    }

    pub async fn patch_state(
        &self
    ) -> () {
        println!("Patching channel state");
        panic!("not implemented!");
    }

    pub async fn publish_message(
        &self
    ) -> () {
        println!("Publishing a channel message");
        panic!("not implemented!");
    }

    // Tokens:

    pub async fn get_token(
        &self
    ) -> () {
        println!("Getting a channel token");
        panic!("not implemented!");
    }

    pub async fn create_token(
        &self
    ) -> () {
        println!("Creating a channel token");
        panic!("not implemented!");
    }

    pub async fn delete_token(
        &self
    ) -> () {
        println!("Deleting a channel token");
        panic!("not implemented!");
    }

    pub async fn set_token_state(
        &self
    ) -> () {
        println!("Setting a channel token's state");
        panic!("not implemented!");
    }

    pub async fn is_token_online(
        &self
    ) -> () {
        println!("Checking if channel token is online");
        panic!("not implemented!");
    }

    pub async fn publish_direct_message(
        &self
    ) -> () {
        println!("Publishing a direct message to a channel token");
        panic!("not implemented!");
    }
}
