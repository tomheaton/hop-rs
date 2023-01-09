use crate::client::APIClient;

pub struct Projects {
    client: APIClient,
}

impl Projects {
    pub fn new(
        client: APIClient
    ) -> Projects {
        return Projects {
            client,
        };
    }

    pub fn get_members(
        &self,
    ) {
        println!("Getting all project members")
    }

    pub fn get_current_member(
        &self,
    ) {
        println!("Getting current project member")
    }

    pub fn get_secrets(
        &self,
    ) {
        println!("Getting all project secrets");
    }

    pub fn create_secret(
        &self,
        name: &str,
        value: String,
    ) {
        println!("Creating a project secret with name: {} and value: {}", name, value);
    }

    pub fn delete_secret(
        &self,
        id: &str,
    ) {
        println!("Deleting a project secret with id: {}", id);
    }
}
