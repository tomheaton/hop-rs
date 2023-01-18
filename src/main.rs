// extern crate hop;
extern crate dotenv;
extern crate rand;

use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use rand::Rng;

use hop::Hop;

use crate::types::ignite::{DeploymentConfig, Image, Resources, RestartPolicy, RuntimeType, Vgpu, VgpuType};
use crate::utils::get_bytes;

pub mod client;
pub mod hop;
pub mod sdks;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() {
    println!("hop-rs");

    dotenv().ok();

    // let my_token = "ptk_xxx";
    // let my_token = env::var("PROJECT_TOKEN").expect("PROJECT_TOKEN needed!");
    let my_token = env::var("PERSONAL_TOKEN").expect("PERSONAL_TOKEN needed!");
    let hop = Hop::new(my_token.as_str());

    // Example: Creating a project secret
    // hop.projects.create_secret(
    //     "RANDOM_NUMBER_69",
    //     rand::thread_rng().gen_range(0..100).to_string(),
    // ).await.unwrap();

    // let r = hop.projects.delete_secret("RANDOM_NUMBER_69").await.unwrap();
    // let r = hop.projects.delete_secret("ps_OTcxNTM4ODY2MzcxMDEwNjM").await.unwrap();
    // println!("{:?}", r);

    // Example: Getting a project's members
    // let members = hop.projects.get_members().await.unwrap();
    // println!("members: {:#?}", members);

    // Example: Creating a project token
    // hop.projects.create_token(1).await.unwrap();

    // Example: user @me
    // let me = hop.users.get_me().await.unwrap();
    // println!("me: {:#?}", me);

    // let pats = hop.users.get_pats().await.unwrap();
    // println!("pats: {:#?}", pats);

    // let pat = hop.users.create_pat("gonkie").await.unwrap();
    // println!("pat: {:#?}", pat);

    // hop.users.delete_pat("pid_OTc0MTgyNjk5NTU3OTI5MDk").await.unwrap();
    // let pats = hop.users.get_pats().await.unwrap();
    // println!("pats: {:#?}", pats);

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
            /*Image {
                name: Some("postgres".to_owned()),
                auth: None,
                gh_repo: None,
            },*/
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
