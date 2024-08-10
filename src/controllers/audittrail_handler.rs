use std::sync::Arc;
use entity::g_audittrail as audittrails;
use std::collections::HashMap;
use serde_json::json;
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
pub async fn update_audittrail_handler(id:i32,_:String,body: audittrails::Model,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
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

fn update_map_audittrails(audittrail: audittrails::Model, body: audittrails::Model, id: i32) -> (HashMap<String, String>, audittrails::ActiveModel) {
    let mut update_query = audittrails::ActiveModel {
        id: Set(id),
        ..Default::default() // Start with default values
    };

    let mut changes = HashMap::new();

    let action = body.action.clone();
    if !action.is_empty() {
        update_query.action = Set(action.clone());
        if audittrail.action != action {
            changes.insert("action".to_string(), action);
        }
    }
    // Handle `details`
    let details = body.details.clone();
    if !details.is_empty() {
        update_query.details = Set(details.clone());
        if audittrail.details != details {
            changes.insert("details".to_string(), details);
        }
    }
    let changes_json= body.changes_json.clone();    
    if audittrail.changes_json!= changes_json {
        update_query.changes_json = Set(changes_json.clone());
        changes.insert("changes_json".to_string(), changes_json.to_string());
    }
    let changes_done_at=body.changes_done_at.clone();
    if audittrail.changes_done_at!= changes_done_at {
        update_query.changes_done_at = Set(changes_done_at.clone());
        changes.insert("changes_done_at".to_string(), changes_done_at.to_string());
    }
    let rule_id=body.rule_id.clone();
    if audittrail.rule_id!= rule_id {
        update_query.rule_id = Set(rule_id.clone());
        changes.insert("rule_id".to_string(), rule_id.to_string());
    }
    
    let timestamp=body.timestamp.clone();
    if audittrail.timestamp!= timestamp {
        changes.insert("timestamp".to_string(), timestamp.to_string());
        update_query.timestamp = Set(audittrail.timestamp.clone());
    }
    let workspace_id=body.workspace_id.clone();        
    if audittrail.workspace_id!= workspace_id {
        changes.insert("workspace_id".to_string(), workspace_id.to_string());
        update_query.workspace_id = Set(workspace_id.clone());
    }
    let sub_resource_id=body.sub_resource_id.clone();        
    if audittrail.sub_resource_id!= sub_resource_id {
        changes.insert("sub_resource_id".to_string(), sub_resource_id.to_string());
        update_query.sub_resource_id = Set(sub_resource_id.clone());
    }
    let resource_id=body.resource_id.clone();        
    if audittrail.resource_id!= resource_id {
        changes.insert("resource_id".to_string(), resource_id.to_string());
        update_query.resource_id = Set(resource_id.clone());
    }
    (changes, update_query)
}

