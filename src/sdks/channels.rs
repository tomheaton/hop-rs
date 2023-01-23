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

        let channels = response["data"]["channels"].to_owned();

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
                response = APIClient::new(
                    self.token.as_str(),
                ).put(
                    format!("/v1/channels/{}", id).as_str(),
                    serde_json::json!({
                        "type": channel_type,
                        "state": state,
                    }),
                ).await.unwrap();
            }
            None => {
                response = APIClient::new(
                    self.token.as_str(),
                ).post(
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

        let stats = response["data"]["stats"].to_owned();

        return Ok(serde_json::from_value(stats).unwrap());
    }

    // FIXME: this is not working
    pub async fn subscribe_tokens(
        &self,
        channel_id: &str,
        tokens: Vec<&str>,
    ) -> Result<(), APIError> {
        println!("Subscribing tokens to channel");

        for token in tokens {
            APIClient::new(
                self.token.as_str(),
            ).put_none(
                format!("/v1/channels/{}/subscribers/{}", channel_id, token).as_str(),
            ).await.unwrap();
        }

        return Ok(());
    }

    // FIXME: this is not working
    pub async fn subscribe_token(
        &self,
        channel_id: &str,
        token_id: &str,
    ) -> Result<(), APIError> {
        println!("Subscribing token to channel");

        APIClient::new(
            self.token.as_str(),
        ).put_none(
            format!("/v1/channels/{}/subscribers/{}", channel_id, token_id).as_str(),
        ).await.unwrap();

        return Ok(());
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

        let tokens = response["data"]["tokens"].to_owned();

        return Ok(serde_json::from_value(tokens).unwrap());
    }*/

    pub async fn set_state(
        &self,
        channel_id: &str,
        state: HashMap<String, String>,
    ) -> Result<(), APIError> {
        println!("Setting channel state");

        APIClient::new(
            self.token.as_str(),
        ).put(
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

        APIClient::new(
            self.token.as_str(),
        ).patch(
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

        APIClient::new(
            self.token.as_str(),
        ).post(
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

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            format!("/v1/channels/tokens/{}", token_id).as_str()
        ).await.unwrap();

        let token = response["data"]["token"].to_owned();

        return Ok(serde_json::from_value(token).unwrap());
    }


    pub async fn create_token(
        &self,
        // TODO: use raw serde_json here to allow any value?
        state: Option<HashMap<String, String>>,
    ) -> Result<ChannelToken, APIError> {
        println!("Creating a channel token");

        // TODO: inline this? (intellisense not available inside the macro)
        let state = state.unwrap_or(HashMap::new());

        let response = APIClient::new(
            self.token.as_str(),
        ).post(
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

        return APIClient::new(
            self.token.as_str(),
        ).delete(
            format!("/v1/channels/tokens/{}", token_id).as_str()
        ).await;
    }

    pub async fn set_token_state(
        &self,
        token_id: &str,
        state: HashMap<String, String>,
    ) -> Result<ChannelToken, APIError> {
        println!("Setting a channel token's state");

        let response = APIClient::new(
            self.token.as_str(),
        ).patch(
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

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
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

        APIClient::new(
            self.token.as_str(),
        ).post(
            format!("/v1/channels/tokens/{}/messages", token_id).as_str(),
            serde_json::json!({
                "e": event,
                "d": data,
            }),
        ).await.unwrap();

        return Ok(());
    }
}
