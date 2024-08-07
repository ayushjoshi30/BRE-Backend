use std::sync::Arc;

use std::collections::HashMap;
use entity::tenants;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::tenants::Entity as TenantEntity;


pub async fn create_tenant_handler(authenticated: bool ,body: tenants::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{

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


pub async fn read_tenant_handler(id: i32, _:bool, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match TenantEntity::find().filter(tenants::Column::Id.eq(id)).one(&*db_pool).await {
        // If the user is empty, return a 404   
        Ok(Some(tenant)) => Ok(warp::reply::json(&tenant)),
        Ok(None) => Err(reject::custom(ResourceNotFound)),

        Err(_) => Err(reject::custom(DatabaseErrorr)),  
    }
}

pub async fn delete_tenant_handler(id:i32,_:bool,db_pool:Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    match tenants::Entity::delete_many()
        .filter(tenants::Column::Id.eq(id.clone()))
        .exec(&*db_pool)
        .await
    {
        Ok(result) if result.rows_affected > 0 => {
            Ok(warp::reply::json(&format!("{} rows deleted", result.rows_affected)))
        }
        Ok(_) => Err(reject::custom(ResourceNotFound)), // Handle case where no rows were affected
        Err(_) => Err(reject::custom(DatabaseErrorr)),
    }
}
pub async fn update_tenant_handler(id:i32,_:bool,body: tenants::Model,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
    let tenant = TenantEntity::find().filter(tenants::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseErrorr))?;

    let tenant = tenant.ok_or(reject::custom(ResourceNotFound))?;
    let changes_map = changes_map_users(tenant.clone(), body.clone());
    let mut update_query = tenants::ActiveModel {
        id: Set(id),
        ..Default::default() // Start with default values
    };

    if let Some(identifier) = body.identifier {
        if !identifier.is_empty() {
            update_query.identifier = Set(Some(identifier));
        }
    }

    if let Some(base_url) = body.base_url {
        if !base_url.is_empty() {
            update_query.base_url = Set(Some(base_url));
        }
    }

    if let Some(workspace_id ) = body.workspace_id {
        if !workspace_id.is_empty() {
            update_query.workspace_id = Set(Some(workspace_id));
        }
    }

    // Execute the update
    tenants::Entity::update(update_query)
        .filter(tenants::Column::Id.eq(id))
        .exec(&*db_pool)
        .await
        .map_err(|_| reject::custom(DatabaseErrorr))?;

    // Retrieve the updated tenant
    match tenants::Entity::find()
        .filter(tenants::Column::Id.eq(id))
        .one(&*db_pool)
        .await
    {
        Ok(Some(tenant)) => Ok(warp::reply::json(&tenant)),
        Ok(None) => Err(reject::custom(ResourceNotFound)),
        Err(_) => Err(reject::custom(DatabaseErrorr)),
    }
}
fn changes_map_users(tenant: tenants::Model, body: tenants::Model) -> HashMap<String, Option<std::option::Option<std::string::String>>>{
    let mut changes = HashMap::new();
    
    if tenant.identifier != body.identifier {
        changes.insert("identifier".to_string(), Some(body.identifier.clone()));
    }
    if tenant.workspace_id != body.workspace_id {
        changes.insert("workspace_id".to_string(), Some(body.workspace_id.clone()));
    }
    if tenant.base_url != body.base_url {
        changes.insert("base_url".to_string(), Some(body.base_url.clone()));
    }
    
    changes
}