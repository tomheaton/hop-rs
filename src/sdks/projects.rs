use crate::client::APIClient;
use crate::client::BASE_URL as URL;
use crate::types::APIError;
use crate::types::projects::Member;

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
        println!("Getting all project members");

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
        println!("Getting current project member");

        let response = self.client.get(
            // format!("/v1/projects/{}/members/@me", project_id).as_str()
            "/v1/projects/@this/members/@me"
        ).await.unwrap();

        let member = response["data"]["member"].to_owned();

        return Ok(serde_json::from_value(member).unwrap());
    }

    // Tokens:

    pub async fn get_tokens(
        &self,
    ) -> Result<serde_json::Value, APIError> {
        println!("Getting all project tokens");

        return self.client.get(
            "/v1/projects/@this/tokens"
        ).await;
    }

    pub async fn create_token(
        &self,
        flags: i32,
    ) -> Result<serde_json::Value, APIError> {
        println!("Creating a project token with flags: {}", flags);

        return self.client.post(
            "/v1/projects/@this/tokens",
            // serde_json::json!(flags),
            serde_json::json!({
                "flags": flags,
            }),
        ).await;
    }

    pub async fn delete_token(
        &self,
        id: &str,
    ) -> Result<(), APIError> {
        println!("Deleting a project token with id: {}", id);

        return self.client.delete(
            format!("/v1/projects/@this/tokens/{}", id).as_str(),
        ).await;
    }

    // Secrets:

    pub async fn get_secrets(
        &self,
    ) -> Result<serde_json::Value, APIError> {
        println!("Getting all project secrets");

        return self.client.get(
            "/v1/projects/@this/secrets"
        ).await;
    }

    pub async fn create_secret(
        &self,
        name: &str,
        value: String,
    ) -> Result<serde_json::Value, APIError> {
        println!("Creating a project secret with name: {} and value: {}", name, value);

        return self.client.put_raw(
            format!("/v1/projects/@this/secrets/{}", name).as_str(),
            value,
        ).await;
    }

    pub async fn delete_secret(
        &self,
        id: &str,
    ) -> Result<(), APIError> {
        println!("Deleting a project secret with id: {}", id);

        return self.client.delete(
            format!("/v1/projects/@this/secrets/{}", id).as_str(),
        ).await;
    }
}
