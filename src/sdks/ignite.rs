use crate::types::ignite::{Config, Deployment};

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

    pub async fn get_deployments() -> () {
        println!("Getting all ignite deployments");
        panic!("not implemented!");
    }

    pub async fn create_deployment(
        deployment: &Deployment,
    ) -> () {
        println!("Creating an ignite deployment: {:?}", deployment);
        panic!("not implemented!");
    }

    pub async fn delete_deployment() -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }
}

impl Deployment {
    pub fn new(
        id: String,
        name: String,
        build_id: String,
        active_build: String,
        active_rollout: String,
        latest_rollout: String,
        created_at: String,
        entrypoint: Vec<String>,
        target_container_count: i64,
        container_count: i64,
        running_container_count: i64,
        metadata: String,
        config: Config,
    ) -> Deployment {
        return Deployment {
            id,
            name,
            build_id,
            active_build,
            active_rollout,
            latest_rollout,
            created_at,
            entrypoint,
            target_container_count,
            container_count,
            running_container_count,
            metadata,
            config,
        };
    }
}
