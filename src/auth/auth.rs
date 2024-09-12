use crate::{error::Error, Result, WebResult};
use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"9d2150d10883d5be069fcb99d3d20b1d";

#[derive(Clone, PartialEq)]


#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    key: String,
    exp: usize,
    user:String,
}

pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| ( headers))
        .and_then(authorize)
}

pub fn create_jwt(username:String) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(1000))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        key: String::from("graviton"),
        exp: expiration as usize,
        user: String::from(username),
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error::JWTTokenCreationError)
}

async fn authorize( headers:  HeaderMap<HeaderValue>) -> WebResult<String> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::JWTTokenError))?;

            if String::from(&decoded.claims.key) != String::from("graviton") {
                return Err(reject::custom(Error::NoPermissionError));
            }
            if decoded.claims.user.is_empty(){
                return Err(reject::custom(Error::ParseTokenError));
            }
            
            Ok(decoded.claims.user)
        }
        Err(e) => return Err(reject::custom(e)),
    }
}

pub fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(Error::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}