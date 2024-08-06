use entity::*;
use std::sync::Arc;
use sea_orm::{Set, DatabaseConnection, ActiveModelTrait};
use warp::{Rejection, Reply, reject, reply};
use serde_json::json;
use crate::error;
use crate::error::Error::WrongCredentialsError;
pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Result<T> = std::result::Result<T, error::Error>;

pub async fn user_data_entry(uid: String, name: String, db_pool: Arc<DatabaseConnection>) -> WebResult<serde_json::Value> {
    let role = if uid == "1" {
        "User"
    } else if uid == "2" {
        "Admin"
    } else {
        "Unknown"
    };
    let user = users::ActiveModel {
        name: Set(name),
        role: Set(String::from(role)),
        ..Default::default()
    };

    let user: users::Model = user.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert user: {:?}", e);
        reject::custom(WrongCredentialsError)
    })?;

    println!("User created with ID: {}, NAME: {}, ROLE: {}", user.id, user.name, user.role);

    Ok(json!({
        "id": user.id,
        "name": user.name,
        "role": user.role,
        "uid": uid,
        "message": "User data entry successful"
    }))
}

// Function to insert tenant data
pub async fn tenant_data_entry(identifier: String, base_url: String, workspace_id: String, db_pool: Arc<DatabaseConnection>) -> WebResult<serde_json::Value> {
    let tenant = tenants::ActiveModel {
        identifier: Set(identifier),
        base_url: Set(base_url),
        workspace_id: Set(workspace_id),
        ..Default::default()
    };

    let tenant: tenants::Model = tenant.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert tenant: {:?}", e);
        reject::custom(WrongCredentialsError)
    })?;

    println!("Tenant created with ID: {}, Identifier: {}, Workspace: {}, Base URL: {}", tenant.id, tenant.identifier, tenant.workspace_id, tenant.base_url);

    Ok(json!({
        "id": tenant.id,
        "identifier": tenant.identifier,
        "workspace_id": tenant.workspace_id,
        "base_url": tenant.base_url,
        "message": "Tenant data entry successful"
    }))
}