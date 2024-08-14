use std::sync::Arc;
use entity::g_rules as rules;
use serde_json::json;
use chrono::NaiveDateTime; // For DateTime handling
use serde_json::{Map, Value};
use std::collections::HashMap;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_rules::Entity as RuleEntity;

pub async fn create_rule_handler(authenticated: String ,body: rules::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    
    print!("Request Authenticated: {}", authenticated);
    let rule = rules::ActiveModel {
        workspace_id: Set(body.workspace_id),
        rule_path: Set(body.rule_path),
        rule_json:Set(body.rule_json),
        created_by_user: Set(body.created_by_user),
        last_updated: Set(body.last_updated),
        draft_file_path: Set(body.draft_file_path),
        draft_file_json:Set(body.draft_file_json),
        is_draft: Set(body.is_draft),
        published_at: Set(body.published_at),
        version: Set(body.version),
        ..Default::default()
    };

    let rule: rules::Model = rule.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert rule: {:?}", e);
        reject::custom(InvalidRequestBodyError)
    })?;

    Ok(warp::reply::json(&rule))
}
pub async fn read_rule_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match RuleEntity::find().filter(rules::Column::Id.eq(id)).one(&*db_pool).await {
        // If the rule is empty, return a 404
        Ok(Some(rule)) => Ok(warp::reply::json(&rule)),
        Ok(None) => Err(reject::custom(ResourceNotFound)),

        Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn read_all_rule_handler(_:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match RuleEntity::find().all(&*db_pool).await {
        // If the rule is empty, return a 404
        Ok(rule) => Ok(warp::reply::json(&rule)),
        Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn update_rule_handler(id:i32,_:String,body: HashMap<String, Value>,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
    let rule = RuleEntity::find().filter(rules::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    let rule = rule.ok_or(reject::custom(ResourceNotFound))?;
    let (changes, rule_model)  = update_map_rules(rule.clone(), body.clone(), id);
    let updated_rule = rule_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    // Construct a response with the changes made
    let response = json!({
        "message": "rule updated successfully",
        "changes": changes,
        "entity": updated_rule
    });

    Ok(warp::reply::json(&response))
}
pub async fn delete_rule_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    let rule = RuleEntity::find().filter(rules::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    let rule = rule.ok_or(reject::custom(ResourceNotFound))?;

    let rule = rules::ActiveModel {
        id: Set(rule.id),
        ..Default::default()
    };

    let _ = rule.delete(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    let response = json!({
        "message": "rule deleted successfully",
        "rule": "Resource Id: ".to_string() + &id.to_string()
    });

    Ok(warp::reply::json(&response))
}
fn update_map_rules(
    rule: rules::Model,
    body: HashMap<String, Value>,
    id: i32,
) -> (HashMap<String, String>, rules::ActiveModel) {
    let mut changes = HashMap::new();

    // Function to get all keys from a JSON object
    

    // Convert HashMap to Map
    let map_body: Map<String, Value> = body.clone().into_iter().collect();

    // Get keys from the body
    let body_keys = get_keys(&Value::Object(map_body));

    // Initialize an ActiveModel to apply updates
    let mut update_query = rules::ActiveModel {
        id: Set(id),
        ..Default::default()
    };

    // Handle "workspace_id"
    if body_keys.contains(&"workspace_id".to_string()) {
        if let Some(Value::Number(workspace_id)) = body.get("workspace_id") {
            if let Some(workspace_id_i64) = workspace_id.as_i64() {
                let workspace_id = workspace_id_i64 as i32;
                if rule.workspace_id != workspace_id {
                    update_query.workspace_id = Set(workspace_id);
                    changes.insert("workspace_id".to_string(), workspace_id.to_string());
                }
            }
        }
    }

    // Handle "rule_path"
    if body_keys.contains(&"rule_path".to_string()) {
        if let Some(Value::String(rule_path)) = body.get("rule_path") {
            if rule.rule_path != *rule_path {
                update_query.rule_path = Set(rule_path.clone());
                changes.insert("rule_path".to_string(), rule_path.clone());
            }
        }
    }

    // Handle "rule_json"
    if body_keys.contains(&"rule_json".to_string()) {
        if let Some(Value::Object(rule_json)) = body.get("rule_json") {
            let rule_json = serde_json::to_value(rule_json).unwrap(); // Convert back to Value
            if rule.rule_json != rule_json {
                update_query.rule_json = Set(rule_json.clone());
                changes.insert("rule_json".to_string(), rule_json.to_string());
            }
        }
    }

    // Handle "created_by_user"
    if body_keys.contains(&"created_by_user".to_string()) {
        if let Some(Value::Number(created_by_user)) = body.get("created_by_user") {
            if let Some(created_by_user_i64) = created_by_user.as_i64() {
                let created_by_user = created_by_user_i64 as i32;
                if rule.created_by_user != created_by_user {
                    update_query.created_by_user = Set(created_by_user);
                    changes.insert("created_by_user".to_string(), created_by_user.to_string());
                }
            }
        }
    }

    // Handle "last_updated"
    if body_keys.contains(&"last_updated".to_string()) {
        if let Some(Value::String(last_updated)) = body.get("last_updated") {
            if let Ok(parsed_date) = last_updated.parse::<NaiveDateTime>() {
                if rule.last_updated != parsed_date {
                    update_query.last_updated = Set(parsed_date);
                    changes.insert("last_updated".to_string(), last_updated.clone());
                }
            }
        }
    }

    // Handle "draft_file_path"
    if body_keys.contains(&"draft_file_path".to_string()) {
        if let Some(Value::String(draft_file_path)) = body.get("draft_file_path") {
            if rule.draft_file_path != *draft_file_path {
                update_query.draft_file_path = Set(draft_file_path.clone());
                changes.insert("draft_file_path".to_string(), draft_file_path.clone());
            }
        }
    }

    // Handle "draft_file_json"
    if body_keys.contains(&"draft_file_json".to_string()) {
        if let Some(Value::Object(draft_file_json)) = body.get("draft_file_json") {
            let draft_file_json = serde_json::to_value(draft_file_json).unwrap(); // Convert back to Value
            if rule.draft_file_json != draft_file_json {
                update_query.draft_file_json = Set(draft_file_json.clone());
                changes.insert("draft_file_json".to_string(), draft_file_json.to_string());
            }
        }
    }

    // Handle "is_draft"
    if body_keys.contains(&"is_draft".to_string()) {
        if let Some(Value::Bool(is_draft)) = body.get("is_draft") {
            if rule.is_draft != *is_draft {
                update_query.is_draft = Set(*is_draft);
                changes.insert("is_draft".to_string(), is_draft.to_string());
            }
        }
    }

    // Handle "published_at"
    if body_keys.contains(&"published_at".to_string()) {
        if let Some(Value::String(published_at)) = body.get("published_at") {
            if let Ok(parsed_date) = published_at.parse::<NaiveDateTime>() {
                if rule.published_at != parsed_date {
                    update_query.published_at = Set(parsed_date);
                    changes.insert("published_at".to_string(), published_at.clone());
                }
            }
        }
    }

    // Handle "version"
    if body_keys.contains(&"version".to_string()) {
        if let Some(Value::String(version)) = body.get("version") {
            if rule.version != *version {
                update_query.version = Set(version.clone());
                changes.insert("version".to_string(), version.clone());
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

