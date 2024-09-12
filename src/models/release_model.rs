use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ReleaseResponse {
    pub id: i32,
    pub version: String,
}