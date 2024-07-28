use crate::models::login_model::{LoginRequest, LoginResponse, Users};
use crate::auth::auth::*;
use crate::error;
use crate::error::Error::WrongCredentialsError;
use warp::{Rejection, Reply, reject, reply};
use serde_json::json;

pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Result<T> = std::result::Result<T, error::Error>;

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    match users
        .iter()
        .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
    {
        Some((uid, user)) => {
            let token = create_jwt(&uid, &Role::from_str(&user.role))
                .map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse { token }))
        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}

pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    // Create a json obj with the uid
    let response_obj = json!({
        "uid": uid,
        "message": "Hello User"
    });

    // Return the json obj as a reply
    Ok(warp::reply::json(&response_obj))
}

pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    // Create a json obj with the uid
    let response_obj = json!({
        "uid": uid,
        "message": "Hello Admin"
    });

    // Return the json obj as a reply
    Ok(warp::reply::json(&response_obj))
}