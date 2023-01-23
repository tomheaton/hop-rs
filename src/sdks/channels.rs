use std::collections::HashMap;

use crate::client::APIClient;
use crate::types::APIError;
use crate::types::channels::{Channel, ChannelToken, ChannelType};

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
        &self,
        channel_type: ChannelType,
        state: Option<HashMap<String, String>>,
    ) -> Result<Channel, APIError> {
        println!("Creating a  channel");

        let response = APIClient::new(
            self.token.as_str(),
        ).post(
            "/v1/channels",
            serde_json::json!({
                "type": channel_type,
                "state": state,
            }),
        ).await.unwrap();

        let channel = response["data"]["channel"].clone();

        return Ok(serde_json::from_value(channel).unwrap());
    }

    pub async fn delete_channel(
        &self,
        channel_id: &str,
    ) -> Result<(), APIError> {
        println!("Deleting a channel");

        return APIClient::new(
            self.token.as_str(),
        ).delete(
            format!("/v1/channels/{}", channel_id).as_str()
        ).await;
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

    /*pub async fn get_tokens(
        &self
    ) -> Result<HashMap<String, i64>, APIError> {
        println!("Getting all channel tokens");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            format!("/v1/channels/{}/tokens", channel_id).as_str()
        ).await.unwrap();

        let tokens = response["data"]["tokens"].clone();

        return Ok(serde_json::from_value(tokens).unwrap());
    }*/

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
        &self,
        token_id: &str,
    ) -> Result<ChannelToken, APIError> {
        println!("Getting a channel token");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            format!("/v1/channels/tokens/{}", token_id).as_str()
        ).await.unwrap();

        let token = response["data"]["token"].clone();

        return Ok(serde_json::from_value(token).unwrap());
    }


    pub async fn create_token(
        &self,
        // TODO: use raw serde_json here to allow any value?
        // state: HashMap<String, String>,
    ) -> Result<ChannelToken, APIError> {
        println!("Creating a channel token");

        let response = APIClient::new(
            self.token.as_str(),
        ).post(
            "/v1/channels/tokens",
            serde_json::json!({
                // FIXME: this does not work
                "type": "lol",
                "state": {
                    "test": "123",
                }
            }),
        ).await.unwrap();

        let token = response["data"]["token"].clone();

        return Ok(serde_json::from_value(token).unwrap());
    }

    pub async fn delete_token(
        &self,
        token_id: &str,
    ) -> Result<(), APIError> {
        println!("Deleting a channel token");

        return APIClient::new(
            self.token.as_str(),
        ).delete(
            format!("/v1/channels/tokens/{}", token_id).as_str()
        ).await;
    }

    pub async fn set_token_state(
        &self
    ) -> () {
        println!("Setting a channel token's state");
        panic!("not implemented!");
    }

    pub async fn is_token_online(
        &self,
        token_id: &str,
    ) -> Result<bool, APIError> {
        println!("Checking if channel token is online");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            format!("/v1/channels/tokens/{}", token_id).as_str()
        ).await.unwrap();

        let token = response["data"]["token"].clone();
        let token = serde_json::from_value::<ChannelToken>(token).unwrap();

        return Ok(token.is_online);
    }

    pub async fn publish_direct_message(
        &self
    ) -> () {
        println!("Publishing a direct message to a channel token");
        panic!("not implemented!");
    }
}
