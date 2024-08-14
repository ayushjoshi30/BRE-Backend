
use std::sync::Arc;
use entity::g_appusers as users;
use std::collections::HashMap;
use serde_json::{Map, Value};
use serde_json::json;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use warp::{reject, reply::Reply};
use sha2::{Digest, Sha256};
use crate::error::Error::*;
use crate::WebResult;
use chrono::NaiveDateTime;
use entity::g_appusers::Entity as UserEntity;


pub async fn create_user_handler(authenticated: String ,body: users::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    let mut hasher = Sha256::new();
    print!("Request Authenticated: {}", authenticated);
    let password = body.password.clone();
    if password == body.password {
        if !password.is_empty() {
            hasher.update(password.as_bytes());
        }
    }
    let hashed_password = format!("{:x}", hasher.finalize());
    let user = users::ActiveModel {
        first_name: Set(body.first_name),
        last_name: Set(body.last_name),
        user_name:Set(body.user_name),
        email: Set(body.email),
        // Set the last login to the current time
        mobile_no: Set(body.mobile_no),
        workspace_id: Set(body.workspace_id),
        created_on_date: Set(body.created_on_date),
        is_active:Set(body.is_active),
        is_deleted: Set(body.is_deleted),
        last_login: Set(body.last_login),
        password: Set(hashed_password),
        is_admin: Set(body.is_admin),
        // Set the last login to the current time
        ..Default::default()
    };

    let user: users::Model = user.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert user: {:?}", e);
        reject::custom(InvalidRequestBodyError)
    })?;

    Ok(warp::reply::json(&user))
}
pub async fn read_user_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().filter(users::Column::Id.eq(id)).one(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(Some(user)) => Ok(warp::reply::json(&user)),
        Ok(None) => Err(reject::custom(ResourceNotFound)),

        Err(_) => Err(reject::custom(DatabaseError)),
    }
}

pub async fn read_all_users_handler(_:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().all(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(users) => Ok(warp::reply::json(&users)),
        Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn update_user_handler(id:i32,_:String,body: HashMap<String, Value>,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
    let user = UserEntity::find().filter(users::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    let user = user.ok_or(reject::custom(ResourceNotFound))?;
    let (changes, user_model)  = update_map_users(user.clone(), body.clone(), id);
    let updated_user = user_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    // Construct a response with the changes made
    let response = json!({
        "message": "user updated successfully",
        "changes": changes,
        "entity": updated_user
    });

    Ok(warp::reply::json(&response))
}

pub async fn delete_user_handler(id: u32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    let user = UserEntity::find().filter(users::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    let user = user.ok_or(reject::custom(ResourceNotFound))?;

    let user = users::ActiveModel {
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


fn update_map_users(
    user: users::Model,
    body: HashMap<String, Value>,
    id: i32,
) -> (HashMap<String, String>, users::ActiveModel) {
    let mut changes = HashMap::new();

    // Function to get all keys from a JSON object
    

    // Convert HashMap to Map
    let map_body: Map<String, Value> = body.clone().into_iter().collect();

    // Get keys from the body
    let body_keys = get_keys(&Value::Object(map_body));

    // Initialize an ActiveModel to apply updates
    let mut update_query = users::ActiveModel {
        id: Set(id),
        ..Default::default()
    };

    // Handle "first_name"
    if body_keys.contains(&"first_name".to_string()) {
        if let Some(Value::String(first_name)) = body.get("first_name") {
            if user.first_name != *first_name {
                update_query.first_name = Set(first_name.clone());
                changes.insert("first_name".to_string(), first_name.clone());
            }
        }
    }

    // Handle "last_name"
    if body_keys.contains(&"last_name".to_string()) {
        if let Some(Value::String(last_name)) = body.get("last_name") {
            if user.last_name != *last_name {
                update_query.last_name = Set(last_name.clone());
                changes.insert("last_name".to_string(), last_name.clone());
            }
        }
    }

    // Handle "user_name"
    if body_keys.contains(&"user_name".to_string()) {
        if let Some(Value::String(user_name)) = body.get("user_name") {
            if user.user_name != *user_name {
                update_query.user_name = Set(user_name.clone());
                changes.insert("user_name".to_string(), user_name.clone());
            }
        }
    }

    // Handle "email"
    if body_keys.contains(&"email".to_string()) {
        if let Some(Value::String(email)) = body.get("email") {
            if user.email != *email {
                update_query.email = Set(email.clone());
                changes.insert("email".to_string(), email.clone());
            }
        }
    }

    // Handle "mobile_no"
    if body_keys.contains(&"mobile_no".to_string()) {
        if let Some(Value::String(mobile_no)) = body.get("mobile_no") {
            if user.mobile_no != *mobile_no {
                update_query.mobile_no = Set(mobile_no.clone());
                changes.insert("mobile_no".to_string(), mobile_no.clone());
            }
        }
    }

    // Handle "created_on_date"
    if body_keys.contains(&"created_on_date".to_string()) {
        if let Some(Value::String(created_on_date)) = body.get("created_on_date") {
            if let Ok(parsed_date) = created_on_date.parse::<NaiveDateTime>() {
                if user.created_on_date != parsed_date {
                    update_query.created_on_date = Set(parsed_date);
                    changes.insert("created_on_date".to_string(), created_on_date.clone());
                }
            }
        }
    }

    // Handle "workspace_id"
    if body_keys.contains(&"workspace_id".to_string()) {
        if let Some(Value::Number(workspace_id)) = body.get("workspace_id") {
            if let Some(workspace_id_i64) = workspace_id.as_i64() {
                if user.workspace_id != workspace_id_i64 as i32 {
                    update_query.workspace_id = Set(workspace_id_i64 as i32);
                    changes.insert("workspace_id".to_string(), workspace_id_i64.to_string());
                }
            }
        }
    }

    // Handle "is_active"
    if body_keys.contains(&"is_active".to_string()) {
        if let Some(Value::Bool(is_active)) = body.get("is_active") {
            if user.is_active != *is_active {
                update_query.is_active = Set(*is_active);
                changes.insert("is_active".to_string(), is_active.to_string());
            }
        }
    }

    // Handle "is_deleted"
    if body_keys.contains(&"is_deleted".to_string()) {
        if let Some(Value::Bool(is_deleted)) = body.get("is_deleted") {
            if user.is_deleted != *is_deleted {
                update_query.is_deleted = Set(*is_deleted);
                changes.insert("is_deleted".to_string(), is_deleted.to_string());
            }
        }
    }

    // Handle "last_login"
    if body_keys.contains(&"last_login".to_string()) {
        if let Some(Value::String(last_login)) = body.get("last_login") {
            if let Ok(parsed_date) = last_login.parse::<NaiveDateTime>() {
                if user.last_login != parsed_date {
                    update_query.last_login = Set(parsed_date);
                    changes.insert("last_login".to_string(), last_login.clone());
                }
            }
        }
    }

    // Handle "password"
    if body_keys.contains(&"password".to_string()) {
        if let Some(Value::String(password)) = body.get("password") {
            if user.password != *password {
                update_query.password = Set(password.clone());
                changes.insert("password".to_string(), password.clone());
            }
        }
    }

    // Handle "is_admin"
    if body_keys.contains(&"is_admin".to_string()) {
        if let Some(Value::Bool(is_admin)) = body.get("is_admin") {
            if user.is_admin != *is_admin {
                update_query.is_admin = Set(*is_admin);
                changes.insert("is_admin".to_string(), is_admin.to_string());
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
    // Extract keys from `body_json`
    // let mut body_keys = HashSet::new();
    // get_keys(&body_json, String::new(), &mut body_keys);

    // // Update fields based on keys present in `body`
    // if body_keys.contains("first_name") {
    //     let first_name = body.first_name.clone();
    //     if !first_name.is_empty() {
    //         update_query.first_name = Set(first_name.clone());
    //         if user.first_name != first_name {
    //             changes.insert("first_name".to_string(), first_name);
    //         }
    //     }
    // }

    // if body_keys.contains("last_name") {
    //     let last_name = body.last_name.clone();
    //     if !last_name.is_empty() {
    //         update_query.last_name = Set(last_name.clone());
    //         if user.last_name != last_name {
    //             changes.insert("last_name".to_string(), last_name);
    //         }
    //     }
    // }

    // if body_keys.contains("user_name") {
    //     let user_name = body.user_name.clone();
    //     if !user_name.is_empty() {
    //         update_query.user_name = Set(user_name.clone());
    //         if user.user_name != user_name {
    //             changes.insert("user_name".to_string(), user_name);
    //         }
    //     }
    // }

    // if body_keys.contains("email") {
    //     let email = body.email.clone();
    //     if !email.is_empty() {
    //         update_query.email = Set(email.clone());
    //         if user.email != email {
    //             changes.insert("email".to_string(), email);
    //         }
    //     }
    // }

    // if body_keys.contains("mobile_no") {
    //     let mobile_no = body.mobile_no.clone();
    //     if !mobile_no.is_empty() {
    //         update_query.mobile_no = Set(mobile_no.clone());
    //         if user.mobile_no != mobile_no {
    //             changes.insert("mobile_no".to_string(), mobile_no);
    //         }
    //     }
    // }

    // if body_keys.contains("workspace_id") {
    //     let workspace_id = body.workspace_id.clone();
    //     if user.workspace_id != workspace_id && user.workspace_id != 0 {
    //         changes.insert("workspace_id".to_string(), workspace_id.to_string());
    //         update_query.workspace_id = Set(workspace_id.clone());
    //     }
    // }

    // if body_keys.contains("is_active") {
    //     let is_active = body.is_active.clone();
    //     if is_active != user.is_active {
    //         update_query.is_active = Set(is_active.clone());
    //         changes.insert("is_active".to_string(), is_active.to_string());
    //     }
    // }

    // if body_keys.contains("is_deleted") {
    //     let is_deleted = body.is_deleted.clone();
    //     if user.is_deleted != is_deleted {
    //         update_query.is_deleted = Set(is_deleted.clone());
    //         changes.insert("is_deleted".to_string(), is_deleted.to_string());
    //     }
    // }

    // if body_keys.contains("password") {
    //     let password = body.password.clone();
    //     if !password.is_empty() {
    //         let mut hasher = Sha256::new();
    //         hasher.update(password.clone());
    //         let hashed_password = format!("{:x}", hasher.finalize());
    //         update_query.password = Set(hashed_password.clone());
    //         if user.password != hashed_password {
    //             changes.insert("password".to_string(), hashed_password);
    //         }
    //     }
    // }

    // if body_keys.contains("is_admin") {
    //     let is_admin = body.is_admin.clone();
    //     if user.is_admin != is_admin {
    //         update_query.is_admin = Set(is_admin.clone());
    //         changes.insert("is_admin".to_string(), is_admin.to_string());
    //     }
    // }

