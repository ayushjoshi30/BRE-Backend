use std::sync::Arc;
use entity::g_configure as configures;
use std::collections::HashMap;
use serde_json::json;
use serde_json::{Value, Map};
use sea_orm::{ ActiveModelTrait,DatabaseConnection, EntityTrait, QueryFilter,Set, ColumnTrait};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_configure::Entity as ConfigureEntity;
pub async fn create_configure_handler(authenticated: String ,body: configures::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{

    print!("Request Authenticated: {}", authenticated);

    let configure = configures::ActiveModel {
        username: Set(body.username),
        password: Set(body.password),
        bucket_name:Set(body.bucket_name),
        // Set the last login to the current time
        ..Default::default()
    };

    let configure: configures::Model = configure.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert configure: {:?}", e);
        reject::custom(InvalidRequestBodyError)
    })?;

    Ok(warp::reply::json(&configure))
}

pub async fn read_configure_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match ConfigureEntity::find()
    .filter(configures::Column::Id.eq(id))
    .one(&*db_pool)
    .await
    {
    // If the configure is found, return it as JSON
    Ok(Some(configure)) => Ok(warp::reply::json(&configure)),

    // If the configure is not found, return a 404 error
    Ok(None) => Err(reject::custom(ResourceNotFound)),

    // If there is a database error, return a generic database error
    Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn update_configure_handler(id:i32,_:String,body: HashMap<String, Value>,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
    let configure = ConfigureEntity::find().filter(configures::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    
    let configure = configure.ok_or(reject::custom(ResourceNotFound))?;
    let (changes, configure_model)  = update_map_configures(configure.clone(), body.clone(), id);
    let updated_configure = configure_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    // Construct a response with the changes made
    let response = json!({
        "message": "configure updated successfully",
        "changes": changes,
        "entity": updated_configure
    });

    Ok(warp::reply::json(&response))
}
pub async fn delete_configure_handler(id:i32,_:String,db_pool:Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    match configures::Entity::delete_many()
        .filter(configures::Column::Id.eq(id.clone()))
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
pub async fn read_all_configure_handler(_:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match ConfigureEntity::find().all(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(configures) => Ok(warp::reply::json(&configures)),
        Err(_) => Err(reject::custom(DatabaseError)),
    }
}



fn update_map_configures(
    configure: configures::Model,
    body: HashMap<String, Value>,
    id: i32,
) -> (HashMap<String, String>, configures::ActiveModel) {
    let mut changes = HashMap::new();

    // Convert HashMap to Map
    let map_body: Map<String, Value> = body.clone().into_iter().collect();

    // Get keys from the body
    let body_keys = get_keys(&Value::Object(map_body));

    // Initialize an ActiveModel to apply updates
    let mut update_query = configures::ActiveModel {
        id: Set(id),
        ..Default::default()
    };

    // Handle "username"
    if body_keys.contains(&"username".to_string()) {
        if let Some(Value::String(username)) = body.get("username") {
            if configure.username != *username {
                update_query.username = Set(username.clone());
                changes.insert("username".to_string(), username.clone());
            }
        }
    }

    // Handle "password"
    if body_keys.contains(&"password".to_string()) {
        if let Some(Value::String(password)) = body.get("password") {
            if configure.password != *password {
                update_query.password = Set(password.clone());
                changes.insert("password".to_string(), password.clone());
            }
        }
    }

    // Handle "file_json"
    if body_keys.contains(&"bucket_name".to_string()) {
        if let Some(Value::String(bucket_name)) = body.get("bucket_name") {
            if configure.bucket_name != *bucket_name {
                update_query.bucket_name = Set(bucket_name.clone());
                changes.insert("bucket_name".to_string(), bucket_name.clone());
            }
        }
    }

    // Return the changes map and updated ActiveModel
    (changes, update_query)
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



