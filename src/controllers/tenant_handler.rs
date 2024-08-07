use std::sync::Arc;

use std::collections::HashMap;
use entity::tenants;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use serde_json::json;
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::tenants::Entity as TenantEntity;


pub async fn create_tenant_handler(authenticated: String ,body: tenants::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{

    print!("Request Authenticated: {}", authenticated);

    let tenant = tenants::ActiveModel {
        identifier: Set(body.identifier),
        workspace_id: Set(body.workspace_id),
        base_url: Set(body.base_url),
        // Set the last login to the current time
        ..Default::default()
    };

    let tenant: tenants::Model = tenant.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert tenant: {:?}", e);
        reject::custom(InvalidRequestBodyError)
    })?;

    Ok(warp::reply::json(&tenant))
}


pub async fn read_tenant_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match TenantEntity::find().filter(tenants::Column::Id.eq(id)).one(&*db_pool).await {
        // If the user is empty, return a 404   
        Ok(Some(tenant)) => Ok(warp::reply::json(&tenant)),
        Ok(None) => Err(reject::custom(ResourceNotFound)),

        Err(_) => Err(reject::custom(DatabaseError)),  
    }
}

pub async fn delete_tenant_handler(id:i32,_:String,db_pool:Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    match tenants::Entity::delete_many()
        .filter(tenants::Column::Id.eq(id.clone()))
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
pub async fn read_all_tenants_handler(_:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match TenantEntity::find().all(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(tenants) => Ok(warp::reply::json(&tenants)),
        Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn update_tenant_handler(id:i32,_:String,body: tenants::Model,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
    let tenant = TenantEntity::find().filter(tenants::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    let tenant = tenant.ok_or(reject::custom(ResourceNotFound))?;
    
    let (changes, tenant_model)  = update_map_tenants(tenant.clone(), body.clone(), id);
    
    let updated_tenant = tenant_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    // Construct a response with the changes made
    let response = json!({
        "message": "Tenant updated successfully",
        "changes": changes,
        "entity": updated_tenant
    });

    Ok(warp::reply::json(&response))
}

fn update_map_tenants(tenant: tenants::Model, body: tenants::Model, id: i32) -> (HashMap<String, Option<std::option::Option<std::string::String>>>, tenants::ActiveModel) {
    let mut update_query = tenants::ActiveModel {
        id: Set(id),
        ..Default::default() // Start with default values
    };

    let mut changes = HashMap::new();

    if let Some(identifier) = body.identifier.clone() {
        if !identifier.is_empty() {
            update_query.identifier = Set(Some(identifier));
            if tenant.identifier != body.identifier {
                changes.insert("identifier".to_string(), Some(body.identifier));
            }
        }
    }

    if let Some(base_url) = body.base_url.clone() {
        if !base_url.is_empty() {
            update_query.base_url = Set(Some(base_url));
            if tenant.base_url != body.base_url {
                changes.insert("base_url".to_string(), Some(body.base_url));
            }
        }
    }

    if let Some(workspace_id ) = body.workspace_id.clone() {
        if !workspace_id.is_empty() {
            update_query.workspace_id = Set(Some(workspace_id));
            if tenant.workspace_id != body.workspace_id {
                changes.insert("workspace_id".to_string(), Some(body.workspace_id));
            }
        }
    }

    (changes, update_query)
}