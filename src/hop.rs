use crate::client::APIClient;
use crate::sdks::projects::Projects;

const DEFAULT_BASE_URL: &str = "https://api.hop.io";

pub struct Hop {
    pub client: APIClient,

    pub ignite: i32,
    pub users: i32,
    pub projects: Projects,
    pub pipe: i32,
    pub registry: i32,
    pub channels: i32,
}

impl Hop {
    pub fn new(
        token: &str
    ) -> Hop {
        let client = APIClient::new();

        let hop = Hop {
            client,

            ignite: 1,
            users: 1,
            projects: Projects::new(client.clone()),
            pipe: 1,
            registry: 1,
            channels: 1,
        };

        return hop;
    }
}
