use serde_json::Value;

use crate::{APIClient, APIError, APIResponseOld};
use crate::types::ignite::{Container, ContainerState, CreateHealthCheckConfig, Deployment, DeploymentConfig, DeploymentLog, HealthCheck, Rollout, StorageStats, UpdateHealthCheckConfig};

pub struct Ignite {
    pub token: String,
    pub client: APIClient,
}

impl Ignite {
    pub fn new(
        token: &str,
    ) -> Ignite {
        return Ignite {
            token: token.to_owned(),
            client: APIClient::new(token),
        };
    }

    // Deployments:

    pub async fn get_deployments(
        &self,
    ) -> Result<Vec<Deployment>, APIError> {
        println!("Getting all ignite deployments");

        let response = self.client.get(
            "/v1/ignite/deployments",
        ).await.unwrap();

        let deployments = response["data"]["deployments"].to_owned();

        return Ok(serde_json::from_value(deployments).unwrap());
    }

    pub async fn get_deployment(
        &self,
        deployment_id: &str,
    ) -> Result<Deployment, APIError> {
        println!("Getting an ignite deployment");

        let response = self.client.get(
            format!("/v1/ignite/deployments/{}", deployment_id).as_str(),
        ).await.unwrap();

        let deployment = response["data"]["deployment"].to_owned();

        return Ok(serde_json::from_value(deployment).unwrap());
    }

    pub async fn get_deployment_by_name(
        &self,
        deployment_name: &str,
    ) -> Result<Deployment, APIError> {
        println!("Getting an ignite deployment by name");

        let response = self.client.get(
            format!("/v1/ignite/deployments/search?name={}", deployment_name).as_str(),
        ).await.unwrap();

        let deployment = response["data"]["deployment"].to_owned();

        return Ok(serde_json::from_value(deployment).unwrap());
    }

    // TODO: this
    pub async fn create_deployment(
        &self,
        deployment: DeploymentConfig,
    ) -> () {
        println!("Creating an ignite deployment: {:?}", deployment);
        panic!("not implemented!");
    }

    pub async fn delete_deployment(
        &self,
        deployment_id: &str,
    ) -> Result<(), APIError> {
        println!("Deleting an ignite deployment");

        self.client.delete(
            format!("/v1/ignite/deployments/{}", deployment_id).as_str(),
        ).await.unwrap();

        return Ok(());
    }

    pub async fn rollout_deployment(
        &self,
        deployment_id: &str,
    ) -> Result<Rollout, APIError> {
        println!("Rolling out an ignite deployment");

        let response = self.client.post(
            format!("/v1/ignite/deployments/{}/rollouts", deployment_id).as_str(),
            Value::Null,
        ).await.unwrap();

        let rollout = response["data"]["rollout"].to_owned();

        return Ok(serde_json::from_value(rollout).unwrap());
    }

    // TODO: this
    pub async fn update_deployment(
        &self,
    ) -> () {
        println!("Updating an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn get_storage_stats(
        &self,
        deployment_id: &str,
    ) -> Result<StorageStats, APIError> {
        println!("Getting an ignite deployment's storage stats");

        let response = self.client.get(
            format!("/v1/ignite/deployments/{}/storage", deployment_id).as_str(),
        ).await.unwrap();

        let storage_stats = response["data"].to_owned();

        return Ok(serde_json::from_value(storage_stats).unwrap());
    }

    pub async fn get_containers(
        &self,
        deployment_id: &str,
    ) -> Result<Vec<Container>, APIError> {
        println!("Deleting an ignite deployment");

        let response = self.client.get(
            format!("/v1/ignite/deployments/{}/containers", deployment_id).as_str(),
        ).await.unwrap();

        let containers = response["data"]["containers"].to_owned();

        return Ok(serde_json::from_value(containers).unwrap());
    }

    // TODO: these
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
        gateway_id: &str,
        domain: &str,
    ) -> Result<(), APIError> {
        println!("Adding a domain to an ignite gateway");

        self.client.post(
            format!("/v1/ignite/gateways/{}/domains", gateway_id).as_str(),
            serde_json::json!({
                "domain": domain,
            }),
        ).await.unwrap();

        return Ok(());
    }

    pub async fn delete_domain(
        &self,
        gateway_id: &str,
    ) -> Result<(), APIError> {
        println!("Deleting an ignite deployment");

        self.client.delete(
            format!("/v1/ignite/gateways/{}/domains", gateway_id).as_str(),
        ).await.unwrap();

        return Ok(());
    }

    // TODO: check this

    /*pub async fn create_domain(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }*/

    // Health Checks:

    pub async fn create_healthcheck(
        &self,
        deployment_id: &str,
        config: CreateHealthCheckConfig,
    ) -> Result<HealthCheck, APIError> {
        println!("Creating a healthcheck");

        let response = self.client.post(
            format!("/v1/ignite/deployments/{}/health-check", deployment_id).as_str(),
            serde_json::json!(config),
        ).await.unwrap();

        let healthcheck = response["data"]["health_check"].to_owned();

        return Ok(serde_json::from_value(healthcheck).unwrap());
    }

    pub async fn update_healthcheck(
        &self,
        deployment_id: &str,
        config: UpdateHealthCheckConfig,
    ) -> Result<(), APIError> {
        println!("Updating a healthcheck");

        let mut config = serde_json::json!(config);
        config = config.as_object().unwrap().iter().fold(serde_json::json!({}), |mut acc, (k, v)| {
            if !v.is_null() {
                acc[k] = v.clone();
            }
            return acc;
        });

        println!("config: {:?}", config);

        self.client.patch(
            format!("/v1/ignite/deployments/{}/health-check", deployment_id).as_str(),
            serde_json::json!(config),
        ).await.unwrap();

        return Ok(());
    }

    // Containers:

    // TODO: this
    pub async fn create_container(
        &self,
    ) -> () {
        println!("Creating an ignite container");
        panic!("not implemented!");
    }

    pub async fn delete_container(
        &self,
        container_id: &str,
        // TODO: add options
        recreate: bool,
        // TODO: should it return this?
    ) -> Result<Option<Container>, APIError> {
        println!("Deleting an ignite deployment");

        if !recreate {
            self.client.delete(
                format!("/v1/ignite/containers/{}", container_id).as_str(),
            ).await.unwrap();

            return Ok(None);
        }

        let response = self.client.delete_with_return(
            format!("/v1/ignite/containers/{}?recreate=true", container_id).as_str(),
            // TODO: add recreate option to query params
            // format!("/v1/ignite/containers/{}", container_id).as_str(),
        ).await.unwrap();

        let container = response["data"]["container"].to_owned();
        println!("container: {:?}", container);

        return Ok(Some(serde_json::from_value(container).unwrap()));
    }

    pub async fn start_container(
        &self,
        container_id: &str,
    ) -> Result<(), APIError> {
        println!("Starting an ignite container");

        self.client.put(
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

        self.client.put(
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

        let response = self.client.get(
            // TODO: add options as query params
            format!("/v1/ignite/containers/{}/logs?offset=0", container_id).as_str(),
        ).await.unwrap();

        let logs = response["data"]["logs"].to_owned();

        return Ok(serde_json::from_value(logs).unwrap());
    }
}
