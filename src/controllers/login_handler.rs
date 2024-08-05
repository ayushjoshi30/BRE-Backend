use std::sync::mpsc::TrySendError;
use std::sync::Arc;

use crate::models::login_model::{LoginRequest, LoginResponse, Users};
use crate::auth::auth::*;
use crate::error;
use crate::error::Error::WrongCredentialsError;
use sea_orm::{Set, ConnectionTrait, DatabaseBackend, DatabaseConnection, QueryResult, Statement, ActiveModelTrait};
use serde::ser::SerializeStruct;
use warp::{Rejection, Reply, reject, reply};
use serde_json::json;
use entity::tenants;

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

pub async fn user_handler(uid: String, db_pool : Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    // Create a json obj with the uid
    let response_obj = json!({
        "uid": uid,
        "message": "Hello User"
    });

    let tenant = tenants::ActiveModel {
        identifier: Set(String::from("ufg")),
        base_url: Set(String::from("https://ufg.kugelblitz.in")),
        workspace_id: Set(String::from("1")),
        ..Default::default()
    };


    let tenant: tenants::Model = tenant.insert(&*db_pool.clone()).await.unwrap();

    println!("Post created with ID: {}, TITLE: {}, Workspace: {}, Base: {}", tenant.id, tenant.identifier, tenant.workspace_id, tenant.base_url);
    // Return the json obj as a reply
    Ok(warp::reply::json(&response_obj))
}

pub async fn admin_handler(uid: String, db_pool : Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    // Create a json obj with the uid
    let response_obj = json!({
        "uid": uid,
        "message": "Hello Admin"
    });

    // Return the json obj as a reply
    Ok(warp::reply::json(&response_obj))
}