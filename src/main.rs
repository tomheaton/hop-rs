extern crate dotenv;
extern crate hop;
extern crate rand;

use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use rand::Rng;

use hop::Hop;
use hop::types::ignite::{DeploymentConfig, Image, Resources, RestartPolicy, RuntimeType};

#[tokio::main]
async fn main() {
    println!("hop");

    dotenv().ok();

    let my_token = env::var("PROJECT_TOKEN").expect("PROJECT_TOKEN needed!");
    // let my_token = env::var("PERSONAL_TOKEN").expect("PERSONAL_TOKEN needed!");
    let hop = Hop::new(my_token.as_str());

    // Example: Creating a deployment
    hop.ignite.create_deployment(
        DeploymentConfig::new(
            "postgres",
            RuntimeType::Ephemeral,
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
                1,
                "1GB",
                vec![],
                /*vec![
                    Vgpu::new(
                        VgpuType::A400,
                        1,
                    )
                ],*/
            ),
            RestartPolicy::Never,
            None,
            None,
        ),
    ).await;//.unwrap();
}
