use crate::types::ignite::{Config, Deployment, DeploymentConfig};

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

    pub async fn get_deployments(
        &self
    ) -> () {
        println!("Getting all ignite deployments");
        panic!("not implemented!");
    }

    pub async fn create_deployment(
        &self,
        // deployment: Deployment,
        deployment: DeploymentConfig,
    ) -> () {
        println!("Creating an ignite deployment: {:?}", deployment);
        // panic!("not implemented!");
    }

    pub async fn delete_deployment(
        &self,
    ) -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }
}
