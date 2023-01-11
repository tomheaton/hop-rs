use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Image {}

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub tag: String,
    pub digest: Digest,
}

#[derive(Deserialize, Debug)]
pub struct Digest {
    pub digest: String,
    pub size: i64,
    pub uploaded: String,
}
