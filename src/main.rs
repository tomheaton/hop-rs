extern crate dotenv;
extern crate hop;
extern crate rand;

use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use rand::Rng;

use hop::Hop;
use hop::types::ignite::{CreateHealthCheckConfig, DeploymentConfig, HealthCheckProtocol, Image, Resources, RestartPolicy, RuntimeType, UpdateHealthCheckConfig};

#[tokio::main]
async fn main() {
    println!("hop");

    dotenv().ok();

    let my_token = env::var("PROJECT_TOKEN").expect("PROJECT_TOKEN needed!");
    // let my_token = env::var("PERSONAL_TOKEN").expect("PERSONAL_TOKEN needed!");
    let hop = Hop::new(my_token.as_str());

    // let health_check = hop.ignite.create_healthcheck(
    //     "deployment_MTAzMjMxNjU1MDg0OTIwODMz",
    //     CreateHealthCheckConfig {
    //         protocol: HealthCheckProtocol::HTTP,
    //         path: "/".to_string(),
    //         port: 0,
    //         interval: 0,
    //         timeout: 0,
    //         initial_delay: 0,
    //         max_retries: 0,
    //     },
    // ).await.unwrap();
    // println!("health_check: {:#?}", health_check);

    hop.ignite.update_healthcheck(
        "deployment_MTAzMjMxNjU1MDg0OTIwODMz",
        UpdateHealthCheckConfig {
            protocol: None,
            path: Some("/aish".to_string()),
            port: None,
            interval: None,
            timeout: None,
            initial_delay: None,
            max_retries: None,
        },
    ).await.unwrap();

    // TODO: this
    // Example: Creating a deployment
    // hop.ignite.create_deployment(
    //     DeploymentConfig::new(
    //         "postgres",
    //         RuntimeType::Ephemeral,
    //         None,
    //         Image::new(
    //             Some("postgres"),
    //             None,
    //             None,
    //         ),
    //         Some(HashMap::from([
    //             ("POSTGRES_PASSWORD", "password")
    //         ])),
    //         Resources::new(
    //             1,
    //             "1GB",
    //             vec![],
    //             /*vec![
    //                 Vgpu::new(
    //                     VgpuType::A400,
    //                     1,
    //                 )
    //             ],*/
    //         ),
    //         RestartPolicy::Never,
    //         None,
    //         None,
    //     ),
    // ).await;//.unwrap();
}
