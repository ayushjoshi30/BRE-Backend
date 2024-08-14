use std::sync::Arc;
use entity::g_workspaces as workspaces;
use std::collections::HashMap;
use serde_json::json;
use serde_json::{Value, Map};
use sea_orm::{ ActiveModelTrait,DatabaseConnection, EntityTrait, QueryFilter,Set, ColumnTrait};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_workspaces::Entity as WorkspaceEntity;
pub async fn create_workspace_handler(authenticated: String ,body: workspaces::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{

    print!("Request Authenticated: {}", authenticated);

    let workspace = workspaces::ActiveModel {
        identifier: Set(body.identifier),
        organisation_name: Set(body.organisation_name),
        organisation_address:Set(body.organisation_address),
        organisation_email: Set(body.organisation_email),
        // Set the last login to the current time

        base_url: Set(body.base_url),
        auth_key:Set(body.auth_key),
        organization_logo: Set(body.organization_logo),
        // Set the last login to the current time
        ..Default::default()
    };

    let workspace: workspaces::Model = workspace.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert workspace: {:?}", e);
        reject::custom(InvalidRequestBodyError)
    })?;

    Ok(warp::reply::json(&workspace))
}

pub async fn read_workspace_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match WorkspaceEntity::find()
    .filter(workspaces::Column::Id.eq(id))
    .one(&*db_pool)
    .await
    {
    // If the workspace is found, return it as JSON
    Ok(Some(workspace)) => Ok(warp::reply::json(&workspace)),

    // If the workspace is not found, return a 404 error
    Ok(None) => Err(reject::custom(ResourceNotFound)),

    // If there is a database error, return a generic database error
    Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn update_workspace_handler(id:i32,_:String,body: HashMap<String, Value>,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
    let workspace = WorkspaceEntity::find().filter(workspaces::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    
    let workspace = workspace.ok_or(reject::custom(ResourceNotFound))?;
    
    let (changes, workspace_model)  = update_map_workspaces(workspace.clone(), body.clone(), id);
    println!("{:?}",workspace_model.clone());
    let updated_workspace = workspace_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    // Construct a response with the changes made
    let response = json!({
        "message": "workspace updated successfully",
        "changes": changes,
        "entity": updated_workspace
    });

    Ok(warp::reply::json(&response))
}
pub async fn delete_workspace_handler(id:i32,_:String,db_pool:Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    match workspaces::Entity::delete_many()
        .filter(workspaces::Column::Id.eq(id.clone()))
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
pub async fn read_all_workspaces_handler(_:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match WorkspaceEntity::find().all(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(workspaces) => Ok(warp::reply::json(&workspaces)),
        Err(_) => Err(reject::custom(DatabaseError)),
    }
}

fn update_map_workspaces(
    workspace: workspaces::Model,
    body: HashMap<String, Value>,
    id: i32,
) -> (HashMap<String, String>, workspaces::ActiveModel) {
    let mut changes = HashMap::new();

    // Convert HashMap to Map
    let map_body: Map<String, Value> = body.clone().into_iter().collect();

    // Get keys from the body
    let body_keys = get_keys(&Value::Object(map_body));

    // Initialize an ActiveModel to apply updates
    let mut update_query = workspaces::ActiveModel {
        id: Set(id),
        ..Default::default()
    };

    // Handle "identifier"
    if body_keys.contains(&"identifier".to_string()) {
        if let Some(Value::String(identifier)) = body.get("identifier") {
            if workspace.identifier != *identifier {
                update_query.identifier = Set(identifier.clone());
                changes.insert("identifier".to_string(), identifier.clone());
            }
        }
    }

    // Handle "base_url"
    if body_keys.contains(&"base_url".to_string()) {
        if let Some(Value::String(base_url)) = body.get("base_url") {
            if workspace.base_url != *base_url {
                update_query.base_url = Set(base_url.clone());
                changes.insert("base_url".to_string(), base_url.clone());
            }
        }
    }

    // Handle "organisation_name"
    if body_keys.contains(&"organisation_name".to_string()) {
        if let Some(Value::String(organisation_name)) = body.get("organisation_name") {
            if workspace.organisation_name != *organisation_name {
                update_query.organisation_name = Set(organisation_name.clone());
                changes.insert("organisation_name".to_string(), organisation_name.clone());
            }
        }
    }

    // Handle "organisation_address"
    if body_keys.contains(&"organisation_address".to_string()) {
        if let Some(Value::String(organisation_address)) = body.get("organisation_address") {
            if workspace.organisation_address != *organisation_address {
                update_query.organisation_address = Set(organisation_address.clone());
                changes.insert("organisation_address".to_string(), organisation_address.clone());
            }
        }
    }

    // Handle "organisation_email"
    if body_keys.contains(&"organisation_email".to_string()) {
        if let Some(Value::String(organisation_email)) = body.get("organisation_email") {
            if workspace.organisation_email != *organisation_email {
                update_query.organisation_email = Set(organisation_email.clone());
                changes.insert("organisation_email".to_string(), organisation_email.clone());
            }
        }
    }

    // Handle "auth_key"
    if body_keys.contains(&"auth_key".to_string()) {
        if let Some(Value::String(auth_key)) = body.get("auth_key") {
            if workspace.auth_key != *auth_key {
                update_query.auth_key = Set(auth_key.clone());
                changes.insert("auth_key".to_string(), auth_key.clone());
            }
        }
    }

    // Handle "organization_logo"
    if body_keys.contains(&"organization_logo".to_string()) {
        if let Some(Value::String(organization_logo)) = body.get("organization_logo") {
            if workspace.organization_logo != *organization_logo {
                update_query.organization_logo = Set(organization_logo.clone());
                changes.insert("organization_logo".to_string(), organization_logo.clone());
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

