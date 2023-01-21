pub struct Channels {
    pub token: String,
}

impl Channels {
    pub fn new(
        token: &str,
    ) -> Channels {
        return Channels {
            token: token.to_owned(),
        };
    }

    // Channels:

    pub async fn get_channels(
        &self
    ) -> () {
        println!("Getting all channels");
        panic!("not implemented!");
    }

    pub async fn get_channel(
        &self
    ) -> () {
        println!("Getting a channel");
        panic!("not implemented!");
    }

    pub async fn create_channel(
        &self
    ) -> () {
        println!("Creating a channel");
        panic!("not implemented!");
    }

    pub async fn delete_channel(
        &self
    ) -> () {
        println!("Deleting a channel");
        panic!("not implemented!");
    }

    pub async fn get_stats(
        &self
    ) -> () {
        println!("Getting a channel's stats");
        panic!("not implemented!");
    }

    pub async fn subscribe_tokens(
        &self
    ) -> () {
        println!("Getting channel subscription tokens");
        panic!("not implemented!");
    }

    pub async fn subscribe_token(
        &self
    ) -> () {
        println!("Getting channel subscription token");
        panic!("not implemented!");
    }

    pub async fn get_tokens(
        &self
    ) -> () {
        println!("Getting all channel tokens");
        panic!("not implemented!");
    }

    pub async fn set_state(
        &self
    ) -> () {
        println!("Setting channel state");
        panic!("not implemented!");
    }

    pub async fn patch_state(
        &self
    ) -> () {
        println!("Patching channel state");
        panic!("not implemented!");
    }

    pub async fn publish_message(
        &self
    ) -> () {
        println!("Publishing a channel message");
        panic!("not implemented!");
    }

    // Tokens:

    pub async fn get_token(
        &self
    ) -> () {
        println!("Getting a channel token");
        panic!("not implemented!");
    }

    pub async fn create_token(
        &self
    ) -> () {
        println!("Creating a channel token");
        panic!("not implemented!");
    }

    pub async fn delete_token(
        &self
    ) -> () {
        println!("Deleting a channel token");
        panic!("not implemented!");
    }

    pub async fn set_token_state(
        &self
    ) -> () {
        println!("Setting a channel token's state");
        panic!("not implemented!");
    }

    pub async fn is_token_online(
        &self
    ) -> () {
        println!("Checking if channel token is online");
        panic!("not implemented!");
    }

    pub async fn publish_direct_message(
        &self
    ) -> () {
        println!("Publishing a direct message to a channel token");
        panic!("not implemented!");
    }
}
