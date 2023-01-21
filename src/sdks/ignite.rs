use crate::types::ignite::DeploymentConfig;

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
        &self
    ) -> () {
        println!("Getting all ignite deployments");
        panic!("not implemented!");
    }

    pub async fn get_deployment(
        &self
    ) -> () {
        println!("Getting all ignite deployments");
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
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn update_deployment(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn get_storage_stats(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
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
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn get_gateway(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn create_gateway(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn add_domain(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
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
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn update_healthcheck(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    // Containers:

    pub async fn get_container_logs(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn delete_container(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn stop_container(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn start_container(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn create_container(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }
}
