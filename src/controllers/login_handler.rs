use std::sync::Arc;
use crate::models::login_model::{LoginRequest, LoginResponse, Users};
use crate::auth::auth::*;
use crate::error;
use crate::error::Error::WrongCredentialsError ;
use sea_orm::{Set, DatabaseConnection, ActiveModelTrait,EntityTrait};
use warp::{Rejection, Reply, reject, reply};
use serde_json::json;
use entity::{tenants,users};
use crate::datafeeding::data_entry::*;
use sea_orm::prelude::*;
pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Result<T> = std::result::Result<T, error::Error>;
use entity::users::Entity as UserEntity;
use entity::tenants::Entity as TenantEntity;
use entity::users::Column as UserColumn;

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    match users
        .iter()
        .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
    {
        Some((uid, user)) => {
            let token = create_jwt(&uid, &Role::from_str(&user.role))
                .map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse { token }))
        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}

pub async fn user_handler(uid: String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    // Determine the role based on the uid
    

    // Call the user data entry function
    let admin_users = UserEntity::find()
        .filter(UserColumn::Role.eq("User"))  // Use the correct column variant
        .all(&*db_pool)
        .await
        .map_err(|e| {
        eprintln!("Error retrieving  users: {:?}", e);
        warp::reject::custom(error::Error::DatabaseErrorr)
    })?;

    // Extract the names of the admin users
    let names: Vec<String> = admin_users.into_iter()
        .map(|user| user.name)
        .collect();

    // Create the response object
    let response_obj = json!({
        "message": "Hello user!",
        "user_names": names
    });

    Ok(warp::reply::json(&response_obj))
}


pub async fn admin_handler(uid: String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    
    // Return combined responses as a JSON object
    let admin_users = UserEntity::find()
        .filter(UserColumn::Role.eq("Admin"))  // Use the correct column variant
        .all(&*db_pool)
        .await
        .map_err(|e| {
        eprintln!("Error retrieving admin users: {:?}", e);
        warp::reject::custom(error::Error::DatabaseErrorr)
    })?;

    // Extract the names of the admin users
    let names: Vec<String> = admin_users.into_iter()
        .map(|user| user.name)
        .collect();

    // Create the response object
    let response_obj = json!({
        "message": "Hello admin!",
        "admin_names": names
    });

    Ok(warp::reply::json(&response_obj))
}
pub async fn set_tenant(uid:String,body: tenants::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    let tenant_response = tenant_data_entry(
        body.identifier,
        body.base_url,
        body.workspace_id,
        db_pool,
    ).await?;
    let identifier = tenant_response.get("identifier")
        .and_then(|r| r.as_str())
        .unwrap_or("Unknown");
    let response_obj = json!({
        "message": format!("Tenant added: {}!", identifier),
        "tenant": tenant_response,
        
    });

    Ok(warp::reply::json(&response_obj))
}
pub async fn new_user(uid: String,body: users::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    let users_response = user_data_entry(uid.clone(), body.name, db_pool.clone()).await?;
    let role = users_response.get("role")
        .and_then(|r| r.as_str())
        .unwrap_or("Unknown");
    let response_obj = json!({
        "message": format!("Hello {}!",role),
        "user": users_response,
    });

    Ok(warp::reply::json(&response_obj))
}

pub async fn view_tenants(uid:String,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match TenantEntity::find().all(&*db_pool).await {
        Ok(tenants) => Ok(warp::reply::json(&tenants)),
        Err(e) => {
            eprintln!("Error retrieving tenants: {:?}", e);
            Err(reject::custom(error::Error::DatabaseErrorr)) // Specific to your defined Error enum
        }
    }
}