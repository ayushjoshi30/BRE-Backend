use std::sync::Arc;
use entity::g_audittrail as audittrails;
use std::collections::HashMap;
use serde_json::json;
use sea_orm::prelude::*;
use serde_json::{Value, Map};
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use sea_orm::{ ActiveModelTrait,DatabaseConnection, EntityTrait, QueryFilter,Set, ColumnTrait};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_audittrail::Entity as AudittrailEntity;
pub async fn create_audittrail_handler(authenticated: String ,body: audittrails::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{

    print!("Request Authenticated: {}", authenticated);

    let audittrail = audittrails::ActiveModel {
        action: Set(body.action),
        details: Set(body.details),
        changes_json:Set(body.changes_json),
        changes_done_at: Set(body.changes_done_at),
        // Set the last login to the current time
        user_id: Set(body.user_id),
        rule_id: Set(body.rule_id),
        timestamp:Set(body.timestamp),
        resource_id:Set(body.resource_id),
        sub_resource_id:Set(body.sub_resource_id),
        workspace_id: Set(body.workspace_id),
        // Set the last login to the current time
        ..Default::default()
    };

    let audittrail: audittrails::Model = audittrail.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert audittrail: {:?}", e);
        reject::custom(InvalidRequestBodyError)
    })?;

    Ok(warp::reply::json(&audittrail))
}

pub async fn read_audittrail_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match AudittrailEntity::find()
    .filter(audittrails::Column::Id.eq(id))
    .one(&*db_pool)
    .await
    {
    // If the audittrail is found, return it as JSON
    Ok(Some(audittrail)) => Ok(warp::reply::json(&audittrail)),

    // If the audittrail is not found, return a 404 error
    Ok(None) => Err(reject::custom(ResourceNotFound)),

    // If there is a database error, return a generic database error
    Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn update_audittrail_handler(id:i32,_:String,body: HashMap<String, Value>,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
    let audittrail = AudittrailEntity::find().filter(audittrails::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    
    let audittrail = audittrail.ok_or(reject::custom(ResourceNotFound))?;
    
    let (changes, audittrail_model)  = update_map_audittrails(audittrail.clone(), body.clone(), id);
    println!("{:?}",audittrail_model.clone());
    let updated_audittrail = audittrail_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    // Construct a response with the changes made
    let response = json!({
        "message": "audittrail updated successfully",
        "changes": changes,
        "entity": updated_audittrail
    });

    Ok(warp::reply::json(&response))
}
pub async fn delete_audittrail_handler(id:i32,_:String,db_pool:Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    match audittrails::Entity::delete_many()
        .filter(audittrails::Column::Id.eq(id.clone()))
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
pub async fn read_all_audittrail_handler(_:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match AudittrailEntity::find().all(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(audittrails) => Ok(warp::reply::json(&audittrails)),
        Err(_) => Err(reject::custom(DatabaseError)),
    }
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

fn update_map_audittrails(
    audittrail: audittrails::Model,
    body: HashMap<String, Value>,
    id: i32,
) -> (HashMap<String, String>, audittrails::ActiveModel) {
    let mut changes = HashMap::new();

    // Convert HashMap to Map
    let map_body: Map<String, Value> = body.clone().into_iter().collect();

    // Get keys from the body
    let body_keys = get_keys(&Value::Object(map_body));

    // Initialize an ActiveModel to apply updates
    let mut update_query = audittrails::ActiveModel {
        id: Set(id),
        ..Default::default()
    };

    // Handle "action"
    if body_keys.contains(&"action".to_string()) {
        if let Some(Value::String(action)) = body.get("action") {
            if audittrail.action != *action {
                update_query.action = Set(action.clone());
                changes.insert("action".to_string(), action.clone());
            }
        }
    }

    // Handle "details"
    if body_keys.contains(&"details".to_string()) {
        if let Some(Value::String(details)) = body.get("details") {
            if audittrail.details != *details {
                update_query.details = Set(details.clone());
                changes.insert("details".to_string(), details.clone());
            }
        }
    }

    // Handle "changes_json"
    if body_keys.contains(&"changes_json".to_string()) {
        if let Some(Value::Object(changes_json)) = body.get("changes_json") {
            let changes_json_string = serde_json::to_string(changes_json).unwrap_or_default();
            let changes_json_value: Json = serde_json::from_str(&changes_json_string).unwrap_or_default();
            if audittrail.changes_json != changes_json_value {
                update_query.changes_json = Set(changes_json_value);
                changes.insert("changes_json".to_string(), changes_json_string);
            }
        }
    }

    // Handle "changes_done_at"
    if body_keys.contains(&"changes_done_at".to_string()) {
        if let Some(Value::String(changes_done_at)) = body.get("changes_done_at") {
            if let Ok(parsed_date) = changes_done_at.parse::<NaiveDateTime>() {
                if audittrail.changes_done_at != parsed_date {
                    update_query.changes_done_at = Set(parsed_date);
                    changes.insert("changes_done_at".to_string(), changes_done_at.clone());
                }
            }
        }
    }

    // Handle "rule_id"
    if body_keys.contains(&"rule_id".to_string()) {
        if let Some(Value::Number(rule_id)) = body.get("rule_id") {
            if let Some(rule_id_i32) = rule_id.as_i64() {
                if audittrail.rule_id != rule_id_i32 as i32 {
                    update_query.rule_id = Set(rule_id_i32 as i32);
                    changes.insert("rule_id".to_string(), rule_id_i32.to_string());
                }
            }
        }
    }
    if body_keys.contains(&"user_id".to_string()) {
        if let Some(Value::Number(user_id)) = body.get("user_id") {
            if let Some(user_id_i32) = user_id.as_i64() {
                if audittrail.user_id != user_id_i32 as i32 {
                    update_query.user_id = Set(user_id_i32 as i32);
                    changes.insert("user_id".to_string(), user_id_i32.to_string());
                }
            }
        }
    }

    // Handle "timestamp"
    if body_keys.contains(&"timestamp".to_string()) {
        if let Some(Value::String(timestamp)) = body.get("timestamp") {
            if let Ok(parsed_timestamp) = timestamp.parse::<DateTime<FixedOffset>>() {
                if audittrail.timestamp != parsed_timestamp {
                    update_query.timestamp = Set(parsed_timestamp);
                    changes.insert("timestamp".to_string(), timestamp.clone());
                }
            }
        }
    }

    // Handle "workspace_id"
    if body_keys.contains(&"workspace_id".to_string()) {
        if let Some(Value::Number(workspace_id)) = body.get("workspace_id") {
            if let Some(workspace_id_i32) = workspace_id.as_i64() {
                if audittrail.workspace_id != workspace_id_i32 as i32 {
                    update_query.workspace_id = Set(workspace_id_i32 as i32);
                    changes.insert("workspace_id".to_string(), workspace_id_i32.to_string());
                }
            }
        }
    }

    // Handle "sub_resource_id"
    if body_keys.contains(&"sub_resource_id".to_string()) {
        if let Some(Value::Number(sub_resource_id)) = body.get("sub_resource_id") {
            if let Some(sub_resource_id_i32) = sub_resource_id.as_i64() {
                if audittrail.sub_resource_id != sub_resource_id_i32 as i32 {
                    update_query.sub_resource_id = Set(sub_resource_id_i32 as i32);
                    changes.insert("sub_resource_id".to_string(), sub_resource_id_i32.to_string());
                }
            }
        }
    }

    // Handle "resource_id"
    if body_keys.contains(&"resource_id".to_string()) {
        if let Some(Value::Number(resource_id)) = body.get("resource_id") {
            if let Some(resource_id_i32) = resource_id.as_i64() {
                if audittrail.resource_id != resource_id_i32 as i32 {
                    update_query.resource_id = Set(resource_id_i32 as i32);
                    changes.insert("resource_id".to_string(), resource_id_i32.to_string());
                }
            }
        }
    }

    // Return the changes map and updated ActiveModel
    (changes, update_query)
}

