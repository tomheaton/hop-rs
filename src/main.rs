// extern crate hop;
extern crate rand;

use hop::Hop;
use rand::Rng;

pub mod client;
pub mod hop;
pub mod sdks;

#[tokio::main]
async fn main() {
    println!("hop-rs");

    let my_token = "ptk_xxx";
    let hop = Hop::new(my_token);

    // Example: Creating a project secret
    hop.projects.create_secret(
        "RANDOM_NUMBER",
        rand::thread_rng().gen_range(0..100).to_string(),
    );

    hop.client.get().await;
}
