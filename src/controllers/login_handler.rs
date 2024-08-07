use crate::auth::auth::*;
use crate::error;
use crate::error::Error::WrongCredentialsError;
use crate::models::login_model::{LoginRequest, LoginResponse, Users};
use warp::{reject, reply, Rejection, Reply};
pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Result<T> = std::result::Result<T, error::Error>;

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    match users
        .iter()
        .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
    {
        Some((uid, user)) => {
            let token =
                create_jwt(&uid, &Role::from_str(&user.role)).map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse { token }))
        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}

