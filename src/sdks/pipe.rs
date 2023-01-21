pub struct Pipe {
    pub token: String,
}

impl Pipe {
    pub fn new(
        token: &str,
    ) -> Pipe {
        return Pipe {
            token: token.to_owned(),
        };
    }

    // Rooms:

    pub async fn get_rooms(
        &self
    ) -> () {
        println!("Getting all rooms");
        panic!("not implemented!");
    }

    // TODO: check this

    /*pub async fn get_room(
        &self
    ) -> () {
        println!("Getting a room");
        panic!("not implemented!");
    }*/

    pub async fn create(
        &self
    ) -> () {
        println!("Creating a room");
        panic!("not implemented!");
    }

    pub async fn delete(
        &self
    ) -> () {
        println!("Deleting a room");
        panic!("not implemented!");
    }
}
