extern crate dotenv;
extern crate hop;
extern crate rand;

use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use rand::Rng;

use hop::Hop;
use hop::types::ignite::{CreateDeploymentConfig, CreateHealthCheckConfig, DeploymentConfig, GatewayConfig, GatewayProtocol, GatewayType, HealthCheckProtocol, Image, Resources, RestartPolicy, RuntimeType, UpdateHealthCheckConfig, VolumeDefinition, VolumeFormat};

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

    // hop.ignite.update_healthcheck(
    //     "deployment_MTAzMjMxNjU1MDg0OTIwODMz",
    //     UpdateHealthCheckConfig {
    //         protocol: None,
    //         path: Some("/aish".to_string()),
    //         port: None,
    //         interval: None,
    //         timeout: None,
    //         initial_delay: None,
    //         max_retries: None,
    //     },
    // ).await.unwrap();

    /*let storage = hop.ignite.get_storage_stats(
        "deployment_MTAzMjMxNjU1MDg0OTIwODMz"
    ).await.unwrap();

    println!("storage: {:#?}", storage);*/

    /*let rollout = hop.ignite.rollout_deployment(
        "deployment_MTAzMjMxNjU1MDg0OTIwODMz"
    ).await.unwrap();

    println!("rollout: {:#?}", rollout);*/

    // let deployments = hop.ignite.get_deployments().await.unwrap();
    // println!("deployments: {:#?}", deployments);
    //
    // let deployment = hop.ignite.get_deployment_by_name(
    //     "redis"
    // ).await.unwrap();
    // println!("deployment: {:#?}", deployment);

    // let new_gateway = hop.ignite.create_gateway(
    //     "deployment_MTAzMjMxNjU1MDg0OTIwODMz",
    //     GatewayConfig {
    //         gateway_type: GatewayType::Internal,
    //         protocol: GatewayProtocol::HTTP,
    //         target_port: 80,
    //         name: "phineas-gateway".to_string(),
    //         internal_domain: Some("phineas".to_string()),
    //     },
    // ).await.unwrap();
    // println!("new_gateway: {:#?}", new_gateway);
    //
    // let gateways = hop.ignite.get_gateways(
    //     "deployment_MTAzMjMxNjU1MDg0OTIwODMz"
    // ).await.unwrap();
    // println!("gateways: {:#?}", gateways);
    //
    // let gateway = hop.ignite.get_gateway(
    //     "gateway_MTAzMjMxNzA5MDQ0NjQxNzk0"
    // ).await.unwrap();
    // println!("gateway: {:#?}", gateway);

    // TODO: this
    // Example: Creating a deployment
    let deployment = hop.ignite.create_deployment(
        CreateDeploymentConfig::new(
            "postgres",
            RuntimeType::Persistent,
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
                Some(vec![]),
                /*vec![
                    Vgpu::new(
                        VgpuType::A400,
                        1,
                    )
                ],*/
            ),
            RestartPolicy::Never,
            /*Some(VolumeDefinition {
                fs: VolumeFormat::EXT4,
                size: "512mb".to_string(),
                mountpath: "/lol".to_string()      ,
            }),*/
            None,
            None,
        ),
    ).await.unwrap();

    println!("deployment: {:#?}", deployment);
}
