use crate::client::APIClient;
use crate::sdks::projects::Projects;

const DEFAULT_BASE_URL: &str = "https://api.hop.io";

pub struct Hop {
    pub client: APIClient,

    pub channels: i32,
    pub ignite: i32,
    pub pipe: i32,
    pub projects: Projects,
    pub registry: i32,
    pub users: i32,
}

impl Hop {
    pub fn new(
        token: &str
    ) -> Hop {
        println!("Creating a new Hop client with token {}", token);

        let client = APIClient::new();

        let hop = Hop {
            client,

            channels: 0,
            ignite: 0,
            pipe: 0,
            projects: Projects::new(client.clone()),
            registry: 0,
            users: 0,
        };

        return hop;
    }
}
