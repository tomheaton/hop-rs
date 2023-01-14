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

    pub async fn get_deployments(
        &self
    ) -> () {
        println!("Getting all ignite deployments");
        panic!("not implemented!");
    }

    pub async fn create_deployment(
        &self,
        deployment: &Deployment,
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

impl Deployment {
    pub fn new(
        id: &str,
        name: &str,
        build_id: &str,
        active_build: &str,
        active_rollout: &str,
        latest_rollout: &str,
        created_at: &str,
        // TODO: fix this
        entrypoint: Vec<&str>,
        target_container_count: i64,
        container_count: i64,
        running_container_count: i64,
        metadata: &str,
        config: Option<Config>,
    ) -> Deployment {
        return Deployment {
            id: id.to_owned(),
            name: name.to_owned(),
            build_id: build_id.to_owned(),
            active_build: active_build.to_owned(),
            active_rollout: active_rollout.to_owned(),
            latest_rollout: latest_rollout.to_owned(),
            created_at: created_at.to_owned(),
            // TODO: fix this
            entrypoint: entrypoint.map(|s| s.to_owned()).collect(),
            target_container_count,
            container_count,
            running_container_count,
            metadata: metadata.to_owned(),
            config: config.unwrap(),
        };
    }
}
