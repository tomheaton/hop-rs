use crate::client::APIClient;
use crate::sdks::channels::Channels;
use crate::sdks::ignite::Ignite;
use crate::sdks::pipe::Pipe;
use crate::sdks::projects::Projects;
use crate::sdks::registry::Registry;
use crate::sdks::users::Users;

// pub const DEFAULT_BASE_URL: &str = "https://tomheaton.dev/api/hello";
pub const DEFAULT_BASE_URL: &str = "https://api.hop.io";

pub struct Hop {
    pub client: APIClient,

    pub channels: Channels,
    pub ignite: Ignite,
    pub pipe: Pipe,
    pub projects: Projects,
    pub registry: Registry,
    pub users: Users,
}

impl Hop {
    pub fn new(
        token: &str
    ) -> Hop {
        println!("Creating a new Hop client with token {}", token);

        let client = APIClient::new(
            DEFAULT_BASE_URL,
            token,
        );

        let hop = Hop {
            client,

            channels: Channels::new(),
            ignite: Ignite::new(),
            pipe: Pipe::new(),
            projects: Projects::new(token),
            registry: Registry::new(),
            users: Users::new(),
        };

        return hop;
    }
}
