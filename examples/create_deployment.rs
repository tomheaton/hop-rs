extern crate dotenv;
extern crate hop;
extern crate rand;

use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use rand::Rng;

use hop::Hop;
use hop::types::ignite::{CreateDeploymentConfig, Image, Resources, RestartPolicy, RuntimeType, UpdateDeploymentConfig, VolumeDefinition, VolumeFormat};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let my_token = "ptk_xxx";
    let my_token = env::var("PROJECT_TOKEN").expect("PROJECT_TOKEN needed!");
    let hop = Hop::new(my_token.as_str());

    // Example: Creating a deployment
    let deployment = hop.ignite.create_deployment(
        CreateDeploymentConfig::new(
            "postgres",
            RuntimeType::Stateful,
            "2022-12-28",
            None,
            Image::new(
                Some("postgres"),
                None,
                None,
            ),
            Some(HashMap::from([
                ("POSTGRES_PASSWORD", "password")
            ])),
            Resources::new(
                1f64,
                "1GB",
                None,
            ),
            RestartPolicy::Never,
            Some(VolumeDefinition {
                fs: VolumeFormat::EXT4,
                size: "1GB".to_string(),
                mountpath: "/lol".to_string(),
            }),
            None,
        ),
    ).await.unwrap();

    println!("deployment: {:#?}", deployment);

    // let updated_deployment = hop.ignite.update_deployment(
    //     deployment.id.as_str(),
    //     // TODO: fix this
    //     //  if None values are removed from the sent config, they won't actually update the config
    //     //  if None values are kept in the sent config, they will be updated to None
    //     UpdateDeploymentConfig {
    //         name: None,
    //         container_strategy: None,
    //         runtime_type: None,
    //         version: None,
    //         cmd: None,
    //         image: None,
    //         env: None,
    //         resources: None,
    //         restart_policy: None,
    //         volume: None,
    //         entrypoint: None,
    //     }
    // ).await.unwrap();
    //
    // println!("updated_deployment: {:#?}", updated_deployment);
}
