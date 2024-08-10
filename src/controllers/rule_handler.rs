use std::sync::Arc;
use entity::g_rules as rules;
use serde_json::json;
use std::collections::HashMap;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_rules::Entity as RuleEntity;

pub async fn create_rule_handler(authenticated: String ,body: rules::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{
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
pub async fn update_rule_handler(id:i32,_:String,body: rules::Model,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
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
fn update_map_rules(rule: rules::Model, body: rules::Model, id: i32) -> (HashMap<String, String>, rules::ActiveModel) {
    let mut update_query = rules::ActiveModel {
        id: Set(id),
        ..Default::default() // Start with default values
    };

    let mut changes = HashMap::new();

    let workspace_id = body.workspace_id.clone();
    if rule.workspace_id != workspace_id {
        changes.insert("workspace_id".to_string(), workspace_id.to_string());
        update_query.workspace_id = Set(workspace_id.clone());
    }
    // Handle `rule_path`
    let rule_path = body.rule_path.clone();
    if !rule_path.is_empty() {
        update_query.rule_path = Set(rule_path.clone());
        if rule.rule_path != rule_path {
            changes.insert("rule_path".to_string(), rule_path);
        }
    }
    let rule_json = body.rule_json.clone();
    if rule.rule_json != rule_json {
        changes.insert("rule_json".to_string(), rule_json.to_string());
        update_query.rule_json = Set(rule_json.clone());
    }
    let created_by_user = body.created_by_user.clone();  
    if rule.created_by_user!= created_by_user {
        changes.insert("created_by_user".to_string(), created_by_user.to_string());
        update_query.created_by_user = Set(created_by_user.clone());
    }
    let last_updated= body.last_updated.clone();
    if rule.last_updated!= last_updated {
        changes.insert("last_updated".to_string(), last_updated.to_string());
        update_query.last_updated = Set(last_updated.clone());
    }
    let draft_file_path = body.draft_file_path.clone();
    if rule.draft_file_path != draft_file_path {
        update_query.draft_file_path = Set(draft_file_path.clone());
        changes.insert("draft_file_path".to_string(), draft_file_path);
    }
    let draft_file_json = body.draft_file_json.clone();
    if rule.draft_file_json != draft_file_json {
        changes.insert("draft_file_json".to_string(), draft_file_json.to_string());
        update_query.draft_file_json = Set(draft_file_json.clone());
    }
    
    let is_draft= body.is_draft.clone();
    if rule.is_draft!= is_draft {
        changes.insert("is_draft".to_string(), is_draft.to_string());
        update_query.is_draft = Set(is_draft.clone());
    }
    let published_at= body.published_at.clone();
    if rule.published_at!= published_at {
        changes.insert("published_at".to_string(), published_at.to_string());
        update_query.published_at = Set(published_at.clone());
    }
    let version = body.version.clone();
    if !version.is_empty() {
        update_query.version = Set(version.clone());
        if rule.version != version {
            changes.insert("version".to_string(), version);
        }
    }
    (changes, update_query)
}