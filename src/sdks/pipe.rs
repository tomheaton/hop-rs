use crate::{APIClient, APIError};
use crate::types::pipe::{DeliveryProtocol, IngestProtocol, Region, Room, RoomOptions, RoomState};

pub struct Pipe {
    pub token: String,
    pub client: APIClient,
}

impl Pipe {
    pub fn new(
        token: &str,
    ) -> Pipe {
        return Pipe {
            token: token.to_owned(),
            client: APIClient::new(token),
        };
    }

    // Rooms:

    pub async fn get_rooms(
        &self
    ) -> Result<Vec<Room>, APIError> {
        println!("Getting all rooms");

        let response = self.client.get(
            "/v1/pipe/rooms"
        ).await.unwrap();

        let rooms = response["data"]["rooms"].to_owned();

        return Ok(serde_json::from_value(rooms).unwrap());
    }

    // TODO: check this
    /*pub async fn get_room(
        &self
    ) -> () {
        println!("Getting a room");
        panic!("not implemented!");
    }*/

    pub async fn create(
        &self,
        name: &str,
        options: RoomOptions,
    ) -> Result<Room, APIError> {
        println!("Creating a room");

        let mut config = serde_json::json!({
                "name": name,

                "ingest_protocol": options.ingest_protocol,
                "region": Region::USEast1,

                "ephemeral": options.ephemeral,

                "delivery_protocols": options.delivery_protocols,
                // "llhls_config": options.hls_config,
            });

        if !options.hls_config.is_none() {
            config["llhls_config"] = serde_json::json!(options.hls_config.unwrap());
        }

        let response = self.client.post(
            "/v1/pipe/rooms",
            config,
        ).await.unwrap();

        println!("response: {:?}", response);

        let mut room = response["data"]["room"].clone();
        room["state"] = serde_json::json!(RoomState::Offline);

        return Ok(serde_json::from_value(room).unwrap());
    }

    pub async fn delete(
        &self,
        room_id: &str,
    ) -> Result<(), APIError> {
        println!("Deleting a room");

        return self.client.delete(
            format!("/v1/pipe/rooms/{}", room_id).as_str()
        ).await;
    }
}
