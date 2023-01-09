extern crate dotenv;
// extern crate hop;
extern crate rand;

use std::env;

use dotenv::dotenv;
use rand::Rng;

use hop::Hop;

pub mod client;
pub mod hop;
pub mod sdks;

#[tokio::main]
async fn main() {
    println!("hop-rs");

    dotenv().ok();

    // let my_token = "ptk_xxx";
    let my_token = env::var("PROJECT_TOKEN").expect("PROJECT_TOKEN needed!");
    println!("my_token here {}", my_token.as_str());
    let my_token = my_token.as_str();
    let hop = Hop::new(my_token);

    // Example: Creating a project secret
    hop.projects.create_secret(
        "RANDOM_NUMBER",
        rand::thread_rng().gen_range(0..100).to_string(),
    ).await;

    hop.projects.get_members().await;

    // hop.client.get().await;
}
