use std::collections::HashMap;

use crate::client::APIClient;
use crate::types::APIError;
use crate::types::channels::{Channel, ChannelToken, ChannelType};

pub struct Channels {
    pub token: String,
    pub client: APIClient,
}

impl Channels {
    pub fn new(
        token: &str,
    ) -> Channels {
        return Channels {
            token: token.to_owned(),
            client: APIClient::new(token),
        };
    }

    // Channels:

    pub async fn get_channels(
        &self
    ) -> Result<Vec<Channel>, APIError> {
        println!("Getting all channels");

        let response = self.client.get(
            "/v1/channels"
        ).await.unwrap();

        let channels = response["data"]["channels"].to_owned();

        return Ok(serde_json::from_value(channels).unwrap());
    }

    pub async fn get_channel(
        &self,
        channel_id: &str,
    ) -> Result<Channel, APIError> {
        println!("Getting a channel");

        let response = self.client.get(
            format!("/v1/channels/{}", channel_id).as_str()
        ).await.unwrap();

        let channel = response["data"]["channel"].to_owned();

        return Ok(serde_json::from_value(channel).unwrap());
    }

    pub async fn create_channel(
        &self,
        channel_type: ChannelType,
        channel_id: Option<&str>,
        // TODO: use raw serde_json here to allow any value?
        state: Option<HashMap<String, String>>,
    ) -> Result<Channel, APIError> {
        println!("Creating a channel");

        let response;

        match channel_id {
            Some(id) => {
                response = self.client.put(
                    format!("/v1/channels/{}", id).as_str(),
                    serde_json::json!({
                        "type": channel_type,
                        "state": state,
                    }),
                ).await.unwrap();
            }
            None => {
                response = self.client.post(
                    "/v1/channels",
                    serde_json::json!({
                        "type": channel_type,
                        "state": state,
                    }),
                ).await.unwrap();
            }
        }

        let channel = response["data"]["channel"].to_owned();

        return Ok(serde_json::from_value(channel).unwrap());
    }

    pub async fn delete_channel(
        &self,
        channel_id: &str,
    ) -> Result<(), APIError> {
        println!("Deleting a channel");

        return self.client.delete(
            format!("/v1/channels/{}", channel_id).as_str()
        ).await;
    }

    pub async fn get_stats(
        &self,
        channel_id: &str,
        // TODO: improve this (ask hop about this)
    ) -> Result<HashMap<String, i64>, APIError> {
        println!("Getting a channel's stats");

        let response = self.client.get(
            format!("/v1/channels/{}/stats", channel_id).as_str()
        ).await.unwrap();

        let stats = response["data"]["stats"].to_owned();

        return Ok(serde_json::from_value(stats).unwrap());
    }

    pub async fn subscribe_tokens(
        &self,
        channel_id: &str,
        tokens: Vec<&str>,
    ) -> Result<(), APIError> {
        println!("Subscribing tokens to channel");

        for token in tokens {
            self.client.put_none(
                format!("/v1/channels/{}/subscribers/{}", channel_id, token).as_str(),
            ).await.unwrap();
        }

        return Ok(());
    }

    pub async fn subscribe_token(
        &self,
        channel_id: &str,
        token_id: &str,
    ) -> Result<(), APIError> {
        println!("Subscribing token to channel");

        self.client.put_none(
            format!("/v1/channels/{}/subscribers/{}", channel_id, token_id).as_str(),
        ).await.unwrap();

        return Ok(());
    }

    /*pub async fn get_tokens(
        &self
    ) -> Result<HashMap<String, i64>, APIError> {
        println!("Getting all channel tokens");

        let response =self.client.get(
            format!("/v1/channels/{}/tokens", channel_id).as_str()
        ).await.unwrap();

        let tokens = response["data"]["tokens"].to_owned();

        return Ok(serde_json::from_value(tokens).unwrap());
    }*/

    pub async fn set_state(
        &self,
        channel_id: &str,
        state: HashMap<String, String>,
    ) -> Result<(), APIError> {
        println!("Setting channel state");

        self.client.put(
            format!("/v1/channels/{}/state", channel_id).as_str(),
            serde_json::json!(state),
        ).await.unwrap();

        return Ok(());
    }

    pub async fn patch_state(
        &self,
        channel_id: &str,
        state: HashMap<String, String>,
    ) -> Result<(), APIError> {
        println!("Patching channel state");

        self.client.patch(
            format!("/v1/channels/{}/state", channel_id).as_str(),
            serde_json::json!(state),
        ).await.unwrap();

        return Ok(());
    }

    pub async fn publish_message(
        &self,
        channel_id: &str,
        event: &str,
        data: HashMap<String, String>,
    ) -> Result<(), APIError> {
        println!("Publishing a channel message");

        self.client.post(
            format!("/v1/channels/{}/messages", channel_id).as_str(),
            serde_json::json!({
                "e": event,
                "d": data,
            }),
        ).await.unwrap();

        return Ok(());
    }

    // Tokens:

    pub async fn get_token(
        &self,
        token_id: &str,
    ) -> Result<ChannelToken, APIError> {
        println!("Getting a channel token");

        let response = self.client.get(
            format!("/v1/channels/tokens/{}", token_id).as_str()
        ).await.unwrap();

        let token = response["data"]["token"].to_owned();

        return Ok(serde_json::from_value(token).unwrap());
    }

    pub async fn create_token(
        &self,
        // TODO: use raw serde_json here to allow any value?
        state: Option<HashMap<String, String>>,
        // state: impl Into<Option<HashMap<String, String>>>,
    ) -> Result<ChannelToken, APIError> {
        // let state = state.into();
        println!("Creating a channel token");

        // TODO: inline this? (intellisense not available inside the macro)
        let state = state.unwrap_or(HashMap::new());

        let response = self.client.post(
            "/v1/channels/tokens",
            serde_json::json!(state),
        ).await.unwrap();

        println!("response: {:?}", response);

        let mut token = response["data"]["token"].clone();
        token["is_online"] = serde_json::json!(true);

        return Ok(serde_json::from_value(token).unwrap());
    }

    pub async fn delete_token(
        &self,
        token_id: &str,
    ) -> Result<(), APIError> {
        println!("Deleting a channel token");

        return self.client.delete(
            format!("/v1/channels/tokens/{}", token_id).as_str()
        ).await;
    }

    pub async fn set_token_state(
        &self,
        token_id: &str,
        state: HashMap<String, String>,
    ) -> Result<ChannelToken, APIError> {
        println!("Setting a channel token's state");

        let response = self.client.patch(
            format!("/v1/channels/tokens/{}", token_id).as_str(),
            serde_json::json!(state),
        ).await.unwrap();

        println!("response: {:?}", response);

        let mut token = response["data"]["token"].clone();
        token["is_online"] = serde_json::json!(true);

        return Ok(serde_json::from_value(token).unwrap());
    }

    pub async fn is_token_online(
        &self,
        token_id: &str,
    ) -> Result<bool, APIError> {
        println!("Checking if channel token is online");

        let response = self.client.get(
            format!("/v1/channels/tokens/{}", token_id).as_str()
        ).await.unwrap();

        let token = response["data"]["token"].to_owned();
        let token = serde_json::from_value::<ChannelToken>(token).unwrap();

        return Ok(token.is_online);
        // return Ok(token.is_online.unwrap());
    }

    pub async fn publish_direct_message(
        &self,
        token_id: &str,
        event: &str,
        data: HashMap<String, String>,
    ) -> Result<(), APIError> {
        println!("Publishing a direct message to a channel token");

        self.client.post(
            format!("/v1/channels/tokens/{}/messages", token_id).as_str(),
            serde_json::json!({
                "e": event,
                "d": data,
            }),
        ).await.unwrap();

        return Ok(());
    }
}
