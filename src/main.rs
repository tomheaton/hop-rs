extern crate dotenv;
extern crate hop;
extern crate rand;

use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use rand::Rng;

use hop::{get_bytes, Hop, types::ignite::{DeploymentConfig, Image, Resources, RestartPolicy, RuntimeType}};
use hop::types::channels::ChannelType;

#[tokio::main]
async fn main() {
    println!("hop");

    dotenv().ok();

    // let my_token = "ptk_xxx";
    let my_token = env::var("PROJECT_TOKEN").expect("PROJECT_TOKEN needed!");
    // let my_token = env::var("PERSONAL_TOKEN").expect("PERSONAL_TOKEN needed!");
    let hop = Hop::new(my_token.as_str());

    // let channels = hop.channels.get_channels().await.unwrap();
    // println!("channels: {:#?}", channels);

    // let channel = hop.channels.get_token("leap_token_c185NjU1YmUxZTJkYTZkYjNiOWE5Njc0NGIxZjcyMDliOF8xMDE3NTU1MTE3Nzc2MzYzNzk").await.unwrap();
    // println!("channel: {:#?}", channel);

    // hop.channels.delete_token("leap_token_c19mMGY2ZjcxOGRmOTk1ODUxZjYyZThlNzRlYjUwZTMyOF8xMDE3NzQ4NDEzNTQxNDE3Mzc").await.unwrap();

    /*let channel = hop.channels.create_channel(
        ChannelType::Private,
        Some("tom2"),
        // None,
        None,
    ).await.unwrap();
    println!("channel: {:#?}", channel);*/

    // let state = HashMap::from([
    //     ("foo".to_string(), "bar".to_string()),
    //     ("baz".to_string(), "qux".to_string()),
    // ]);
    // let token = hop.channels.create_token(Some(state)).await.unwrap();
    // println!("token: {:#?}", token);
    // let on = hop.channels.is_token_online(token.id.as_str()).await.unwrap();
    // println!("on: {:#?}", on);

    // hop.channels.delete_channel("tom").await.unwrap();

    // let token = hop.channels.create_token().await.unwrap();
    // println!("token: {:#?}", token);

    // Example: Creating a project secret
    // hop.projects.create_secret(
    //     "RANDOM_NUMBER",
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

    // let pat = hop.users.create_pat("tomheaton").await.unwrap();
    // println!("pat: {:#?}", pat);

    // hop.users.delete_pat("pid_OTc0MTgyNjk5NTU3OTI5MDk").await.unwrap();
    // let pats = hop.users.get_pats().await.unwrap();
    // println!("pats: {:#?}", pats);

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
