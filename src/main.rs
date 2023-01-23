extern crate dotenv;
extern crate hop;
extern crate rand;

use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use rand::Rng;

use hop::{get_bytes, Hop};
use hop::types::channels::ChannelType;
use hop::types::ignite::{DeploymentConfig, Image, Resources, RestartPolicy, RuntimeType};

#[tokio::main]
async fn main() {
    println!("hop");

    dotenv().ok();

    let my_token = env::var("PROJECT_TOKEN").expect("PROJECT_TOKEN needed!");
    // let my_token = env::var("PERSONAL_TOKEN").expect("PERSONAL_TOKEN needed!");
    let hop = Hop::new(my_token.as_str());

    let tokens = Vec::from([
        "leap_token_c19iZDBlMmFkZTI2OGMzNTQ0NjVkZWQzNDBkM2M1OTNkOF8xMDIwNzY0NzU5Mjg2NTc5NTA",
        "leap_token_c19kYzA3NTIxODExYzJmMDAzMjE5ZGI3OGExNTRmODNmMV8xMDIwNzU1NDIxNjI1NTkwMDk",
    ]);

    // TODO: check me
    // hop.channels.subscribe_token("testing", tokens[0]).await.unwrap();
    // hop.channels.subscribe_tokens("testing", tokens).await.unwrap();

    // hop.channels.patch_state(
    //     "channel_MTAyMDY2MjgzNDAwODM5MTkw",
    //     state,
    // ).await.unwrap();

    /*let x = hop.channels.publish_message(
        "channel_MTAyMDY2MjgzNDAwODM5MTkw",
        "tom",
        HashMap::new(),
    ).await.unwrap();*/

    let state = HashMap::from([
        // ("foo".to_string(), "bar".to_string()),
        // ("baz".to_string(), "qux".to_string()),
        // ("tom".to_string(), "heaton".to_string()),
        ("jeff".to_string(), "heaton".to_string()),
    ]);

    let token = hop.channels.create_token(Some(state)).await.unwrap();
    // let token = hop.channels.create_token(state).await.unwrap();
    let token = hop.channels.create_token(None).await.unwrap();

    // Example: Creating a deployment
    /*hop.ignite.create_deployment(
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
    ).await;//.unwrap();*/
}
