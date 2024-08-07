use std::collections::HashMap;
use std::sync::Arc;

use chrono::NaiveDateTime;
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use serde_json::json;
use sha2::{Digest, Sha256};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::users::Entity as UserEntity;


pub async fn create_user_handler(authenticated: bool ,body: users::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{

    print!("Request Authenticated: {}", authenticated);

    let null_date_time = NaiveDateTime::from_timestamp(0, 0);

    // Hash the password before storing it
    let mut hasher = Sha256::new();
    hasher.update(body.password.as_bytes());
    let hashed_password = format!("{:x}", hasher.finalize());

    let user = users::ActiveModel {
        username: Set(body.username),
        workspace_id: Set(body.workspace_id),
        password: Set(hashed_password),
        role: Set(body.role),
        last_login: Set(null_date_time), // Set the last login to the current time
        ..Default::default()
    };

    let user: users::Model = user.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert user: {:?}", e);
        reject::custom(WrongCredentialsError)
    })?;

    Ok(warp::reply::json(&user))
}


pub async fn read_user_handler(id: i32, _:bool, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().filter(users::Column::Id.eq(id)).one(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(Some(user)) => Ok(warp::reply::json(&user)),
        Ok(None) => Err(reject::custom(ResourceNotFound)),

        Err(_) => Err(reject::custom(DatabaseErrorr)),
    }
}

pub async fn read_all_users_handler(_:bool, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().all(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(users) => Ok(warp::reply::json(&users)),
        Err(_) => Err(reject::custom(DatabaseErrorr)),
    }
}

pub async fn update_user_handler(id: u32, _:bool, body: users::Model, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    let user = UserEntity::find().filter(users::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseErrorr))?;

    let user = user.ok_or(reject::custom(ResourceNotFound))?;

    // create a map of changes made to user

    let changes_map = changes_map_users(user.clone(), body.clone());

    let user_model = users::ActiveModel {
        id: Set(user.id),
        username: Set(body.username),
        workspace_id: Set(body.workspace_id),
        password: Set(body.password),
        role: Set(body.role),
        last_login: Set(user.last_login),
        ..Default::default()
    };

    let updated_user = user_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseErrorr))?;

    // Construct a response with the changes made
    let response = json!({
        "message": "User updated successfully",
        "changes": changes_map,
        "user": updated_user
    });

    Ok(warp::reply::json(&response))
}

pub async fn delete_user_handler(id: u32, _:bool, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    let user = UserEntity::find().filter(users::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseErrorr))?;

    let user = user.ok_or(reject::custom(ResourceNotFound))?;

    let user = users::ActiveModel {
        id: Set(user.id),
        ..Default::default()
    };

    let _ = user.delete(&*db_pool).await.map_err(|_| reject::custom(DatabaseErrorr))?;

    let response = json!({
        "message": "User deleted successfully",
        "user": "Resource Id: ".to_string() + &id.to_string()
    });

    Ok(warp::reply::json(&response))
}

fn changes_map_users(user: users::Model, body: users::Model) -> HashMap<String, String> {
    let mut changes = HashMap::new();
    if user.username != body.username {
        changes.insert("username".to_string(), body.username);
    }
    if user.workspace_id != body.workspace_id {
        changes.insert("workspace_id".to_string(), body.workspace_id.to_string());
    }
    if user.password != body.password {
        changes.insert("password".to_string(), body.password);
    }
    if user.role != body.role {
        changes.insert("role".to_string(), body.role);
    }
    if user.last_login != NaiveDateTime::from_timestamp(0, 0) {
        changes.insert("last_login".to_string(), body.last_login.to_string());
    }
    changes
}