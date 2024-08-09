use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
    #[error("invalid request body")]
    InvalidRequestBodyError,
    #[error("database error")]
    DatabaseError,
    #[error("Resource not found")]
    ResourceNotFound,
    #[error("Unable to parse token")]
    ParseTokenError,
    #[error("Invalid Username")]
    UsernotfoundError,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::WrongCredentialsError => (StatusCode::FORBIDDEN, e.to_string()),
            Error::NoPermissionError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::ResourceNotFound => (StatusCode::NOT_FOUND, e.to_string()),
            Error::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Error::InvalidRequestBodyError => (StatusCode::BAD_REQUEST, e.to_string()),
            Error::NoAuthHeaderError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::InvalidAuthHeaderError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::ParseTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::UsernotfoundError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
        )
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}