use serde::Deserialize;

use crate::client::APIClient;
use crate::types::APIError;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub username: String,
    pub email_verified: bool,
    pub mfa_enabled: bool,
    pub totp_enabled: bool,
    pub webauthn_enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct Pat {
    pub id: String,
    pub pat: String,
    pub created_at: String,
    pub name: Option<String>,
}

pub struct Users {
    pub token: String,
}

impl Users {
    pub fn new(
        token: &str
    ) -> Users {
        return Users {
            token: token.to_owned(),
        };
    }

    pub async fn get_me(
        &self,
    ) -> Result<User, APIError> {
        println!("Getting user information");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            "/v1/users/@me"
        ).await.unwrap();

        let me = response["data"]["user"].clone();

        return Ok(serde_json::from_value(me).unwrap());
    }

    pub async fn get_pats(
        &self,
    ) -> Result<Vec<Pat>, APIError> {
        println!("Getting user pats");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            "/v1/users/@me/pats"
        ).await.unwrap();

        let pats = response["data"]["pats"].clone();

        return Ok(serde_json::from_value(pats).unwrap());
    }

    pub async fn create_pat(
        &self,
        name: &str,
    ) -> Result<Pat, APIError> {
        println!("Creating a user pat with name: {}", name);

        // TODO: fix invalid route error (actually just dumb)
        let response = APIClient::new(
            self.token.as_str(),
        ).post(
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
        println!("Deleting a user pat with id: {}", id);

        // TODO: fix invalid route error (actually just dumb)
        APIClient::new(
            self.token.as_str(),
        ).delete(
            format!("/v1/users/@me/pats/{}", id).as_str()
        ).await.unwrap();

        return Ok(());
    }
}
