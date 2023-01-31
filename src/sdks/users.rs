use serde::Deserialize;

use crate::client::APIClient;
use crate::types::APIError;
use crate::types::users::{Pat, User};

pub struct Users {
    pub token: String,
    pub client: APIClient,
}

impl Users {
    pub fn new(
        token: &str
    ) -> Users {
        return Users {
            token: token.to_owned(),
            client: APIClient::new(token),
        };
    }

    // Me:

    pub async fn get_me(
        &self,
    ) -> Result<User, APIError> {
        let response = self.client.get(
            "/v1/users/@me"
        ).await.unwrap();

        let me = response["data"]["user"].clone();

        return Ok(serde_json::from_value(me).unwrap());
    }

    // Pats:

    pub async fn get_pats(
        &self,
    ) -> Result<Vec<Pat>, APIError> {
        let response = self.client.get(
            "/v1/users/@me/pats"
        ).await.unwrap();

        let pats = response["data"]["pats"].clone();

        return Ok(serde_json::from_value(pats).unwrap());
    }

    pub async fn create_pat(
        &self,
        name: &str,
    ) -> Result<Pat, APIError> {
        // TODO: fix invalid route error (actually just dumb)
        let response = self.client.post(
            "/v1/users/@me/pats",
            // serde_json::json!(name),
            serde_json::json!({
                "name": name,
            }),
        ).await.unwrap();

        let pat = response["data"]["pat"].clone();

        return Ok(serde_json::from_value(pat).unwrap());
    }

    pub async fn delete_pat(
        &self,
        id: &str,
    ) -> Result<(), APIError> {
        // TODO: fix invalid route error (actually just dumb)
        self.client.delete(
            format!("/v1/users/@me/pats/{}", id).as_str()
        ).await.unwrap();

        return Ok(());
    }
}
