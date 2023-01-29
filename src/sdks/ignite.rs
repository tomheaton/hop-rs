use crate::{APIClient, APIError, APIResponseOld, get_bytes};
use crate::types::ignite::{Container, ContainerState, CreateDeploymentConfig, CreateHealthCheckConfig, Deployment, DeploymentConfig, DeploymentLog, Gateway, GatewayConfig, GatewayType, HealthCheck, Rollout, RuntimeType, StorageStats, UpdateHealthCheckConfig};

const SIX_MB_IN_BYTES: i64 = 6 * 1024 * 1024;

const VERSIONS: [&str; 4] = ["2022-05-17", "2022-10-19", "2022-12-12", "2022-12-28"];

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
        config: CreateDeploymentConfig,
    ) -> Result<Deployment, APIError> {
        println!("Creating an ignite deployment: {:?}", config);

        // TODO: should this be here? it is here to match js sdk typed version
        if !VERSIONS.contains(&config.version.as_str()) {
            println!("Invalid version. Valid versions are: {:?}", VERSIONS);
            return Err(APIError);
        }

        if (get_bytes(config.resources.ram.as_str())) <= SIX_MB_IN_BYTES {
            println!("Allocated memory must be greater than 6MB when creating a deployment.");
            return Err(APIError);
        }

        if config.volume.is_some() && config.runtime_type != RuntimeType::Stateful {
            println!("Cannot create a deployment with a volume that is not stateful.");
            return Err(APIError);
        }

        let mut data = serde_json::json!(config);

        if config.volume.is_none() {
            // TODO: check this works
            data.as_object_mut().unwrap().remove("volume");
        }

        let response = self.client.post(
            "/v1/ignite/deployments",
            serde_json::json!(data),
        ).await.unwrap();

        let deployment = response["data"]["deployment"].to_owned();

        return Ok(serde_json::from_value(deployment).unwrap());
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
            serde_json::Value::Null,
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

    // Gateways:

    pub async fn get_gateways(
        &self,
        deployment_id: &str,
    ) -> Result<Vec<Gateway>, APIError> {
        println!("Getting all ignite gateways");

        let response = self.client.get(
            format!("/v1/ignite/deployments/{}/gateways", deployment_id).as_str(),
        ).await.unwrap();

        let gateways = response["data"]["gateways"].to_owned();

        return Ok(serde_json::from_value(gateways).unwrap());
    }

    pub async fn get_gateway(
        &self,
        gateway_id: &str,
    ) -> Result<Gateway, APIError> {
        println!("Getting an ignite gateway");

        let response = self.client.get(
            format!("/v1/ignite/gateways/{}", gateway_id).as_str(),
        ).await.unwrap();

        let gateway = response["data"]["gateway"].to_owned();

        return Ok(serde_json::from_value(gateway).unwrap());
    }

    // TODO: this
    pub async fn create_gateway(
        &self,
        deployment_id: &str,
        config: GatewayConfig,
    ) -> Result<Gateway, APIError> {
        println!("Creating an ignite gateway");

        let mut data = serde_json::json!({
            "type": config.gateway_type,
            "protocol": config.protocol,
            "target_port": config.target_port,
            "name": config.name,
        });

        match config.gateway_type {
            GatewayType::Internal => {
                if config.internal_domain.is_some() {
                    data["internal_domain"] = serde_json::json!(config.internal_domain.unwrap());
                } else {
                    panic!("internal_domain is required for internal gateways");
                }
            }
            _ => {}
        }

        let response = self.client.post(
            format!("/v1/ignite/deployments/{}/gateways", deployment_id).as_str(),
            data,
        ).await.unwrap();

        let gateway = response["data"]["gateway"].to_owned();

        return Ok(serde_json::from_value(gateway).unwrap());
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
