use std::sync::Arc;

use serde_json::Value as Json;
use chrono::NaiveDateTime;
use entity::g_releases as releases;
use crate::controllers::workspace_handler::*;
use std::collections::HashMap;
use serde_json::json;
use crate::models::workspace_model::WorkspaceResponse;
use serde_json::{Value, Map};
use entity::g_workspaces as workspaces;
use entity::g_workspaces::Entity as WorkspaceEntity;
use sea_orm::{ ActiveModelTrait,DatabaseConnection, EntityTrait, QueryFilter,Set, ColumnTrait};
use warp::{http::StatusCode,reject, reply::Reply,hyper::body::to_bytes};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_releases::Entity as ReleaseEntity;
use crate::models::release_model::ReleaseResponse;
use entity::g_appusers as users;
use entity::g_appusers::Entity as UserEntity;
pub async fn create_release_handler(username: String ,body: releases::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    let user_result = UserEntity::find()
        .filter(users::Column::UserName.eq(username.clone()))
        .one(&*db_pool)
        .await;

    // Extract the user ID
    let (user_id,workspace_id) = match user_result {
        Ok(Some(user)) => (user.id, user.workspace_id),
        Ok(None) => return Err(reject::not_found()), // User not found
        Err(_) => return Err(reject::custom(InvalidRequestBodyError)), // Database error
    };
    let release = releases::ActiveModel {
        version: Set(body.version),
        file_path: Set(body.file_path),
        file_json:Set(body.file_json),
        created_at: Set(body.created_at),
        workspace_id: Set(workspace_id),
        // Set the last login to the current time

        is_released: Set(body.is_released),
        released_date:Set(body.released_date),
        created_by_user: Set(user_id),
        // Set the last login to the current time
        ..Default::default()
    };

    let release: releases::Model = release.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert release: {:?}", e);
        reject::custom(InvalidRequestBodyError)
    })?;

    Ok(warp::reply::json(&release))
}

pub async fn read_release_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match ReleaseEntity::find()
    .filter(releases::Column::Id.eq(id))
    .one(&*db_pool)
    .await
    {
    // If the release is found, return it as JSON
    Ok(Some(release)) => Ok(warp::reply::json(&release)),

    // If the release is not found, return a 404 error
    Ok(None) => Err(reject::custom(ResourceNotFound)),

    // If there is a database error, return a generic database error
    Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn update_release_handler(id:i32,_:String,body: HashMap<String, Value>,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
    let release = ReleaseEntity::find().filter(releases::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    
    let release = release.ok_or(reject::custom(ResourceNotFound))?;
    let (changes, release_model)  = update_map_releases(release.clone(), body.clone(), id);
    let updated_release = release_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    // Construct a response with the changes made
    let response = json!({
        "message": "release updated successfully",
        "changes": changes,
        "entity": updated_release
    });

    Ok(warp::reply::json(&response))
}
pub async fn delete_release_handler(id:i32,_:String,db_pool:Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    match releases::Entity::delete_many()
        .filter(releases::Column::Id.eq(id.clone()))
        .exec(&*db_pool)
        .await
    {
        Ok(result) if result.rows_affected > 0 => {
            Ok(warp::reply::json(&format!("{} rows deleted", result.rows_affected)))
        }
        Ok(_) => Err(reject::custom(ResourceNotFound)), // Handle case where no rows were affected
        Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn read_all_release_handler(username:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    let result = read_workspace_handler(username.clone(), db_pool.clone()).await;
    // Extract the response body from the Warp reply
    let response_body = match result {
        Ok(reply) => {
            let bytes = to_bytes(reply.into_response().into_body()).await.unwrap_or_default();
            String::from_utf8(bytes.to_vec()).unwrap_or_default()
        },
        Err(_) => return Err(warp::reject::not_found()), // Handle errors appropriately
    };
    // Deserialize the response into WorkspaceResponse
    let workspace_response: WorkspaceResponse = serde_json::from_str(&response_body).unwrap_or_else(|_| {
        // Handle deserialization errors appropriately
        eprintln!("Failed to deserialize response");
        WorkspaceResponse { id: -1, name: "Unknown".to_string() }
    });
    let workspace_id = workspace_response.id;
    let query = WorkspaceEntity::find()
            .filter(workspaces::Column::Id.eq(workspace_id))
            .find_also_related(ReleaseEntity)
            .all(&*db_pool)
            .await
            .map_err(|_| warp::reject::not_found())?;
    let mut releases=Vec::new();
    for (_,related_releases) in query{
        if let Some(release) = related_releases {
            releases.push(ReleaseResponse {
                id: release.id,
                version: release.version.to_string(), // Adjust according to your field name
            });
        }
    }   
    let response = serde_json::to_string(&releases).unwrap_or_else(|_| "[]".to_string());
    // Now you can use the workspace_id to fetch related rules or perform other actions
    // For now, let's just return it as a simple example response
    Ok(warp::reply::with_status(response, StatusCode::OK))
}

fn get_keys(value: &Value) -> Vec<String> {
    let mut keys = Vec::new();
    if let Value::Object(map) = value {
        for (key, val) in map {
            keys.push(key.clone());
            keys.extend(get_keys(val)); // Recursively get keys
        }
    }
    keys
}

fn update_map_releases(
    release: releases::Model,
    body: HashMap<String, Value>,
    id: i32,
) -> (HashMap<String, String>, releases::ActiveModel) {
    let mut changes = HashMap::new();

    // Convert HashMap to Map
    let map_body: Map<String, Value> = body.clone().into_iter().collect();

    // Get keys from the body
    let body_keys = get_keys(&Value::Object(map_body));

    // Initialize an ActiveModel to apply updates
    let mut update_query = releases::ActiveModel {
        id: Set(id),
        ..Default::default()
    };

    // Handle "version"
    if body_keys.contains(&"version".to_string()) {
        if let Some(Value::String(version)) = body.get("version") {
            if release.version != *version {
                update_query.version = Set(version.clone());
                changes.insert("version".to_string(), version.clone());
            }
        }
    }

    // Handle "file_path"
    if body_keys.contains(&"file_path".to_string()) {
        if let Some(Value::String(file_path)) = body.get("file_path") {
            if release.file_path != *file_path {
                update_query.file_path = Set(file_path.clone());
                changes.insert("file_path".to_string(), file_path.clone());
            }
        }
    }

    // Handle "file_json"
    if body_keys.contains(&"file_json".to_string()) {
        if let Some(Value::Object(file_json)) = body.get("file_json") {
            let file_json_string = serde_json::to_string(file_json).unwrap_or_default();
            let file_json_value: Json = serde_json::from_str(&file_json_string).unwrap_or_default();
            if release.file_json != file_json_value {
                update_query.file_json = Set(file_json_value);
                changes.insert("file_json".to_string(), file_json_string);
            }
        }
    }

    // Handle "created_at"
    if body_keys.contains(&"created_at".to_string()) {
        if let Some(Value::String(created_at)) = body.get("created_at") {
            if let Ok(parsed_date) = created_at.parse::<NaiveDateTime>() {
                if release.created_at != parsed_date {
                    update_query.created_at = Set(parsed_date);
                    changes.insert("created_at".to_string(), created_at.clone());
                }
            }
        }
    }

    // Handle "is_released"
    if body_keys.contains(&"is_released".to_string()) {
        if let Some(Value::Bool(is_released)) = body.get("is_released") {
            if release.is_released != *is_released {
                update_query.is_released = Set(*is_released);
                changes.insert("is_released".to_string(), is_released.to_string());
            }
        }
    }

    // Handle "released_date"
    if body_keys.contains(&"released_date".to_string()) {
        if let Some(Value::String(released_date)) = body.get("released_date") {
            if let Ok(parsed_date) = released_date.parse::<NaiveDateTime>() {
                if release.released_date != parsed_date {
                    update_query.released_date = Set(parsed_date);
                    changes.insert("released_date".to_string(), released_date.clone());
                }
            }
        }
    }

    // Handle "created_by_user"
    if body_keys.contains(&"created_by_user".to_string()) {
        if let Some(Value::Number(created_by_user)) = body.get("created_by_user") {
            if let Some(created_by_user_i32) = created_by_user.as_i64() {
                if release.created_by_user != created_by_user_i32 as i32 {
                    update_query.created_by_user = Set(created_by_user_i32 as i32);
                    changes.insert("created_by_user".to_string(), created_by_user_i32.to_string());
                }
            }
        }
    }

    // Return the changes map and updated ActiveModel
    (changes, update_query)
}



