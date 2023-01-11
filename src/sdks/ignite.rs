pub struct Ignite {
    pub token: String,
}

impl Ignite {
    pub fn new(
        token: &str,
    ) -> Ignite {
        return Ignite {
            token: token.to_owned(),
        };
    }

    pub async fn get_deployments() -> () {
        println!("Getting all ignite deployments");
        panic!("not implemented!");
    }

    pub async fn create_deployment() -> () {
        println!("Creating an ignite deployment");
        panic!("not implemented!");
    }

    pub async fn delete_deployment() -> () {
        println!("Deleting an ignite deployment");
        panic!("not implemented!");
    }
}
