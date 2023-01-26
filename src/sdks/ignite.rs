use crate::{APIClient, APIError};
use crate::types::ignite::{ContainerState, DeploymentConfig, DeploymentLog};

pub struct Ignite {
    pub token: String,
}

impl Ignite {
    pub fn new(
        token: &str,
    ) -> Ignite {
        return Ignite {
            token: token.to_owned(),
        };
    }

    // Deployments:

    pub async fn get_deployments(
        &self,
    ) -> () {
        println!("Getting all ignite deployments");
        panic!("not implemented!");
    }

    pub async fn get_deployment(
        &self,
    ) -> () {
        println!("Getting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn create_deployment(
        &self,
        deployment: DeploymentConfig,
    ) -> () {
        println!("Creating an ignite deployment: {:?}", deployment);
        panic!("not implemented!");
    }

    pub async fn delete_deployment(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn rollout_deployment(
        &self,
    ) -> () {
        println!("Rolling out an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn update_deployment(
        &self,
    ) -> () {
        println!("Updating an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn get_storage_stats(
        &self,
    ) -> () {
        println!("Getting an ignite deployment's storage stats");
        panic!("not implemented!");
    }

    pub async fn get_containers(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    // Gateways:

    pub async fn get_gateways(
        &self,
    ) -> () {
        println!("Getting all ignite gateways");
        panic!("not implemented!");
    }

    pub async fn get_gateway(
        &self,
    ) -> () {
        println!("Getting an ignite gateway");
        panic!("not implemented!");
    }

    pub async fn create_gateway(
        &self,
    ) -> () {
        println!("Creating an ignite gateway");
        panic!("not implemented!");
    }

    pub async fn add_domain(
        &self,
    ) -> () {
        println!("Adding a domain to an ignite gateway");
        panic!("not implemented!");
    }

    // TODO: check these

    /*pub async fn create_domain(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }*/

    /*pub async fn delete_domain(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }*/

    // Health Checks:

    pub async fn create_healthcheck(
        &self,
    ) -> () {
        println!("Creating a healthcheck");
        panic!("not implemented!");
    }

    pub async fn update_healthcheck(
        &self,
    ) -> () {
        println!("Updating a healthcheck");
        panic!("not implemented!");
    }

    // Containers:

    pub async fn create_container(
        &self,
    ) -> () {
        println!("Creating an ignite container");
        panic!("not implemented!");
    }

    pub async fn delete_container(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn start_container(
        &self,
        container_id: &str,
    ) -> Result<(), APIError> {
        println!("Starting an ignite container");

        APIClient::new(
            self.token.as_str(),
        ).put(
            format!("/v1/ignite/containers/{}/state", container_id).as_str(),
            serde_json::json!({
                "preferred_state": ContainerState::Running,
            }),
        ).await.unwrap();

        return Ok(());
    }

    pub async fn stop_container(
        &self,
        container_id: &str,
    ) -> Result<(), APIError> {
        println!("Stopping an ignite container");

        APIClient::new(
            self.token.as_str(),
        ).put(
            format!("/v1/ignite/containers/{}/state", container_id).as_str(),
            serde_json::json!({
                "preferred_state": ContainerState::Stopped
            }),
        ).await.unwrap();

        return Ok(());
    }

    pub async fn get_container_logs(
        &self,
        container_id: &str,
        // TODO: add options
        // options: ()
    ) -> Result<Vec<DeploymentLog>, APIError> {
        println!("Getting an ignite container's logs");

        let response = APIClient::new(
            self.token.as_str(),
        ).get(
            // TODO: add options as query params
            format!("/v1/ignite/containers/{}/logs?offset=0", container_id).as_str(),
        ).await.unwrap();

        let logs = response["data"]["logs"].to_owned();

        return Ok(serde_json::from_value(logs).unwrap());
    }
}
