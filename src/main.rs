// extern crate hop;
extern crate dotenv;
// extern crate rand;

use std::env;

use dotenv::dotenv;
// use rand::Rng;

use hop::Hop;

use crate::types::ignite::Deployment;

pub mod client;
pub mod hop;
pub mod sdks;
pub mod types;

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

    // TESTING:
    // hop.ignite.create_deployment(
    //     // TODO: fix me
    //     Deployment::new(
    //         "1", "2",
    //         "3",
    //         "4",
    //         "5",
    //         "6",
    //         "7",
    //         vec!["8"],
    //         9,
    //         10,
    //         11,
    //         "12",
    //     )
    // ).await.unwrap();
}
