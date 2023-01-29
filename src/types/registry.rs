use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub tag: String,
    pub digest: Digest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Digest {
    pub digest: String,
    pub size: i64,
    pub uploaded: String,
}
