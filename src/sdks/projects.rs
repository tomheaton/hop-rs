use crate::client::APIClient;
use crate::types::APIError;
use crate::types::projects::{Member, Secret, Token};

pub struct Projects {
    pub token: String,
    pub client: APIClient,
}

impl Projects {
    pub fn new(
        token: &str
    ) -> Projects {
        return Projects {
            token: token.to_owned(),
            client: APIClient::new(token),
        };
    }

    // Projects:

    pub async fn get_members(
        &self,
    ) -> Result<Vec<Member>, APIError> {
        let response = self.client.get(
            "/v1/projects/@this/members"
        ).await.unwrap();

        let members = response["data"]["members"].to_owned();

        return Ok(serde_json::from_value(members).unwrap());
    }

    pub async fn get_current_member(
        &self,
        // TODO: add this
        // project_id: &str,
    ) -> Result<Member, APIError> {
        let response = self.client.get(
            // format!("/v1/projects/{}/members/@me", project_id).as_str()
            "/v1/projects/@this/members/@me"
        ).await.unwrap();

        let member = response["data"]["member"].to_owned();

        return Ok(serde_json::from_value(member).unwrap());
    }

    // TODO: fix tokens (ask hop)
    // Tokens:

    pub async fn get_tokens(
        &self,
    ) -> Result<Vec<Token>, APIError> {
        let response = self.client.get(
            "/v1/projects/@this/tokens"
        ).await.unwrap();

        let tokens = response["data"]["tokens"].to_owned();

        return Ok(serde_json::from_value(tokens).unwrap());
    }

    pub async fn create_token(
        &self,
        // TODO: create util to create flags
        flags: i32,
    ) -> Result<Token, APIError> {
        let response = self.client.post(
            "/v1/projects/@this/tokens",
            // serde_json::json!(flags),
            serde_json::json!({
                "flags": flags,
            }),
        ).await.unwrap();

        let token = response["data"]["token"].to_owned();

        return Ok(serde_json::from_value(token).unwrap());
    }

    pub async fn delete_token(
        &self,
        id: &str,
    ) -> Result<(), APIError> {
        self.client.delete(
            format!("/v1/projects/@this/tokens/{}", id).as_str(),
        ).await.unwrap();

        return Ok(());
    }

    // Secrets:

    pub async fn get_secrets(
        &self,
    ) -> Result<Vec<Secret>, APIError> {
        let response = self.client.get(
            "/v1/projects/@this/secrets"
        ).await.unwrap();

        let secrets = response["data"]["secrets"].to_owned();

        return Ok(serde_json::from_value(secrets).unwrap());
    }

    pub async fn create_secret(
        &self,
        name: &str,
        value: String,
    ) -> Result<Secret, APIError> {
        let response = self.client.put_raw(
            format!("/v1/projects/@this/secrets/{}", name).as_str(),
            value,
        ).await.unwrap();

        let secret = response["data"]["secret"].to_owned();

        return Ok(serde_json::from_value(secret).unwrap());
    }

    pub async fn delete_secret(
        &self,
        id: &str,
    ) -> Result<(), APIError> {
        self.client.delete(
            format!("/v1/projects/@this/secrets/{}", id).as_str(),
        ).await.unwrap();

        return Ok(());
    }
}
