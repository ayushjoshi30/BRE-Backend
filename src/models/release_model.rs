use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ReleaseResponse {
    pub id: i32,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadyReleaseRequest {
    pub rules: Vec<i32>, // Assuming rule IDs are of type i32
    pub version: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublishReleaseRequest {
    pub version: String,
}