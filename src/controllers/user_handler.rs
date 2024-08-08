use std::collections::HashMap;
use std::sync::Arc;

use chrono::NaiveDateTime;
use entity::g_appusers;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use serde_json::json;
use sha2::{Digest, Sha256};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_appusers::Entity as UserEntity;


pub async fn create_user_handler(_: String ,body: g_appusers::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{



    let null_date_time = NaiveDateTime::from_timestamp(0, 0);

    // Hash the password before storing it
    let mut hasher = Sha256::new();
    if let Some(password) = body.password.clone() {
        if !password.is_empty() {
            hasher.update(password.as_bytes());
        }
    }
    // hasher.update(Some(body.password.clone()).as_bytes());
    let hashed_password = format!("{:x}", hasher.finalize());

    let user = g_appusers::ActiveModel {
        username: Set(body.username),
        workspace_id: Set(body.workspace_id),
        password: Set(Some(hashed_password)),
        last_login: Set(null_date_time), // Set the last login to the current time
        ..Default::default()
    };

    let user: g_appusers::Model = user.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert user: {:?}", e);
        reject::custom(WrongCredentialsError)
    })?;

    Ok(warp::reply::json(&user))
}


pub async fn read_user_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().filter(g_appusers::Column::Id.eq(id)).one(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(Some(user)) => Ok(warp::reply::json(&user)),
        Ok(None) => Err(reject::custom(ResourceNotFound)),

        Err(_) => Err(reject::custom(DatabaseError)),
    }
}

pub async fn read_all_g_appusers_handler(_:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().all(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(g_appusers) => Ok(warp::reply::json(&g_appusers)),
        Err(_) => Err(reject::custom(DatabaseError)),
    }
}

pub async fn update_user_handler(id: u32, _:String, body: g_appusers::Model, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    let user = UserEntity::find().filter(g_appusers::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    let user = user.ok_or(reject::custom(ResourceNotFound))?;
    // create a map of changes made to user
    let (changes_map, user_model) = update_map_g_appusers(user, body, id as i32);

    let updated_user = user_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    // Construct a response with the changes made
    let response = json!({
        "message": "User updated successfully",
        "changes": changes_map,
        "entity": updated_user
    });

    Ok(warp::reply::json(&response))
}

pub async fn delete_user_handler(id: u32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    let user = UserEntity::find().filter(g_appusers::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    let user = user.ok_or(reject::custom(ResourceNotFound))?;

    let user = g_appusers::ActiveModel {
        id: Set(user.id),
        ..Default::default()
    };

    let _ = user.delete(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    let response = json!({
        "message": "User deleted successfully",
        "user": "Resource Id: ".to_string() + &id.to_string()
    });

    Ok(warp::reply::json(&response))
}

fn update_map_g_appusers(user: g_appusers::Model, body: g_appusers::Model, id: i32) -> (HashMap<String, Option<std::option::Option<std::string::String>>>, g_appusers::ActiveModel) {
    let mut update_query = g_appusers::ActiveModel {
        id: Set(id),
        ..Default::default() // Start with default values
    };

    let mut changes = HashMap::new();

    if let Some(workspace_id) = body.workspace_id.clone() {
        if !workspace_id.is_empty() {
            update_query.workspace_id = Set(Some(workspace_id));
            if user.workspace_id != body.workspace_id {
                changes.insert("workspace_id".to_string(), Some(body.workspace_id));
            }
        }
    }

    if let Some(username) = body.username.clone() {
        if !username.is_empty() {
            update_query.username = Set(Some(username));
            if user.username != body.username {
                changes.insert("username".to_string(), Some(body.username));
            }
        }
    }


    (changes, update_query)
}