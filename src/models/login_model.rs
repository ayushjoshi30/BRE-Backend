use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use warp::Filter;
use std::convert::Infallible;

#[derive(Clone)]
pub struct User {
    #[allow(dead_code)]
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}
#[derive(Deserialize)]


#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub type Users = Arc<HashMap<String, User>>;

pub fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

pub fn init_users() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User {
            uid: String::from("1"),
            email: String::from("user@gmail.com"),
            pw: String::from("1234"),
            role: String::from("User"),
        },
    );
    map.insert(
        String::from("2"),
        User {
            uid: String::from("2"),
            email: String::from("admin@gmail.com"),
            pw: String::from("4321"),
            role: String::from("Admin"),
        },
    );
    map
}