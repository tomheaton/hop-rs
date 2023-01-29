extern crate dotenv;
extern crate hop;
extern crate rand;

use std::env;

use dotenv::dotenv;
use rand::Rng;

use hop::Hop;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let my_token = "ptk_xxx";
    let my_token = env::var("PROJECT_TOKEN").expect("PROJECT_TOKEN needed!");
    let hop = Hop::new(my_token.as_str());

    // Example: Creating a project secret
    let secret = hop.projects.create_secret(
        "RANDOM_NUMBER",
        rand::thread_rng().gen_range(0..100).to_string(),
    ).await.unwrap();

    println!("secret: {:#?}", secret);
}
