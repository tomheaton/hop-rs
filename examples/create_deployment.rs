extern crate dotenv;
extern crate hop;
extern crate rand;

use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use rand::Rng;

use hop::Hop;
use hop::types::ignite::{CreateDeploymentConfig, Image, Resources, RestartPolicy, RuntimeType, VolumeDefinition, VolumeFormat};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let my_token = "ptk_xxx";
    let my_token = env::var("PROJECT_TOKEN").expect("PROJECT_TOKEN needed!");
    let hop = Hop::new(my_token.as_str());

    // Example: Creating a deployment
    let deployment = hop.ignite.create_deployment(
        CreateDeploymentConfig::new(
            "3postgres",
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
}
