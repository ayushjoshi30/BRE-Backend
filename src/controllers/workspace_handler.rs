use std::sync::Arc;
use entity::g_workspaces as workspaces;
use std::collections::HashMap;
use serde_json::json;
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
pub async fn update_workspace_handler(id:i32,_:String,body: workspaces::Model,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
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

fn update_map_workspaces(workspace: workspaces::Model, body: workspaces::Model, id: i32) -> (HashMap<String, String>, workspaces::ActiveModel) {
    let mut update_query = workspaces::ActiveModel {
        id: Set(id),
        ..Default::default() // Start with default values
    };

    let mut changes = HashMap::new();

    let identifier = body.identifier.clone();
    if !identifier.is_empty() {
        update_query.identifier = Set(identifier.clone());
        if workspace.identifier != identifier {
            changes.insert("identifier".to_string(), identifier);
        }
    }
    // Handle `base_url`
    let base_url = body.base_url.clone();
    if !base_url.is_empty() {
        update_query.base_url = Set(base_url.clone());
        if workspace.base_url != base_url {
            changes.insert("base_url".to_string(), base_url);
        }
    }
    let organisation_name = body.organisation_name.clone();
    if !organisation_name.is_empty() {
        update_query.organisation_name = Set(organisation_name.clone());
        if workspace.organisation_name != organisation_name {
            changes.insert("organisation_name".to_string(), organisation_name);
        }
    }
    let organisation_address = body.organisation_address.clone();
    if !organisation_address.is_empty() {
        update_query.organisation_address = Set(organisation_address.clone());
        if workspace.organisation_address!= organisation_address {
            changes.insert("organisation_address".to_string(), organisation_address);
        }
    }
    let organisation_email= body.organisation_email.clone();
    if!organisation_email.is_empty() {
        update_query.organisation_email = Set(organisation_email.clone());
        if workspace.organisation_email!= organisation_email {
            changes.insert("organisation_email".to_string(), organisation_email);
        }
    }
    let auth_key=body.auth_key.clone();
    if!auth_key.is_empty() {
        update_query.auth_key = Set(auth_key.clone());
        if workspace.auth_key!= auth_key {
            changes.insert("auth_key".to_string(), auth_key);
        }
    }
    let organization_logo=body.organization_logo.clone();
    if!organization_logo.is_empty() {
        update_query.organization_logo = Set(organization_logo.clone());
        if workspace.organization_logo!= organization_logo {
            changes.insert("organization_logo".to_string(), organization_logo);
        }
    }

    (changes, update_query)
}

