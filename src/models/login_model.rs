use serde::{Deserialize, Serialize};

#[derive(Deserialize,Clone)]
pub struct LoginRequest {
    pub username: String,
    pub pw: String,
}
#[derive(Deserialize)]


#[derive(Serialize,Clone)]
pub struct LoginResponse {
    pub token: String,
}
