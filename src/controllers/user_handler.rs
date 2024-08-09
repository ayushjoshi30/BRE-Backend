use std::sync::Arc;
use entity::g_appusers as users;
use std::collections::HashMap;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use serde_json::json;
use warp::{reject, reply::Reply};
use sha2::{Digest, Sha256};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_appusers::Entity as UserEntity;


pub async fn create_user_handler(authenticated: String ,body: users::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    let mut hasher = Sha256::new();
    print!("Request Authenticated: {}", authenticated);
    let password = body.password.clone();
    if password == body.password {
        if !password.is_empty() {
            hasher.update(password.as_bytes());
        }
    }
    let hashed_password = format!("{:x}", hasher.finalize());
    let user = users::ActiveModel {
        first_name: Set(body.first_name),
        last_name: Set(body.last_name),
        user_name:Set(body.user_name),
        email: Set(body.email),
        // Set the last login to the current time
        mobile_no: Set(body.mobile_no),
        workspace_id: Set(body.workspace_id),
        created_on_date: Set(body.created_on_date),
        is_active:Set(body.is_active),
        is_deleted: Set(body.is_deleted),
        last_login: Set(body.last_login),
        password: Set(hashed_password),
        is_admin: Set(body.is_admin),
        // Set the last login to the current time
        ..Default::default()
    };

    let user: users::Model = user.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert user: {:?}", e);
        reject::custom(InvalidRequestBodyError)
    })?;

    Ok(warp::reply::json(&user))
}
pub async fn read_user_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().filter(users::Column::Id.eq(id)).one(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(Some(user)) => Ok(warp::reply::json(&user)),
        Ok(None) => Err(reject::custom(ResourceNotFound)),

        Err(_) => Err(reject::custom(DatabaseError)),
    }
}

pub async fn read_all_users_handler(_:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().all(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(users) => Ok(warp::reply::json(&users)),
        Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn update_user_handler(id:i32,_:String,body: users::Model,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
    let user = UserEntity::find().filter(users::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    let user = user.ok_or(reject::custom(ResourceNotFound))?;
    let (changes, user_model)  = update_map_users(user.clone(), body.clone(), id);
    let updated_user = user_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    // Construct a response with the changes made
    let response = json!({
        "message": "user updated successfully",
        "changes": changes,
        "entity": updated_user
    });

    Ok(warp::reply::json(&response))
}

pub async fn delete_user_handler(id: u32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    let user = UserEntity::find().filter(users::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    let user = user.ok_or(reject::custom(ResourceNotFound))?;

    let user = users::ActiveModel {
        id: Set(user.id),
        ..Default::default()
    };

    let _ = user.delete(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    let response = json!({
        "message": "User deleted successfully",
        "user": "Resource Id: ".to_string() + &id.to_string()
    });

    Ok(warp::reply::json(&response))
}


fn update_map_users(user: users::Model, body: users::Model, id: i32) -> (HashMap<String, String>, users::ActiveModel) {
    let mut update_query = users::ActiveModel {
        id: Set(id),
        ..Default::default() // Start with default values
    };

    let mut changes = HashMap::new();

    let first_name = body.first_name.clone();
    if !first_name.is_empty() {
        update_query.first_name = Set(first_name.clone());
        if user.first_name != first_name {
            changes.insert("first_name".to_string(), first_name);
        }
    }
    // Handle `last_name`
    let last_name = body.last_name.clone();
    if !last_name.is_empty() {
        update_query.last_name = Set(last_name.clone());
        if user.last_name != last_name {
            changes.insert("last_name".to_string(), last_name);
        }
    }
    let user_name = body.user_name.clone();
    if !user_name.is_empty() {
        update_query.user_name = Set(user_name.clone());
        if user.user_name != user_name {
            changes.insert("user_name".to_string(), user_name);
        }
    }
    let email = body.email.clone();
    if !email.is_empty() {
        update_query.email = Set(email.clone());
        if user.email!= email {
            changes.insert("email".to_string(), email);
        }
    }
    let mobile_no= body.mobile_no.clone();
    if!mobile_no.is_empty() {
        update_query.mobile_no = Set(mobile_no.clone());
        if user.mobile_no!= mobile_no {
            changes.insert("mobile_no".to_string(), mobile_no);
        }
    }
    // let current_time=current_time.clone();
    // let created_on_date=body.created_on_date.clone();
    // if created_on_date!=current_time.to_string() {
    //     update_query.created_on_date = Set(created_on_date.clone());
    //     if user.created_on_date!= created_on_date {
    //         changes.insert("created_on_date".to_string(), created_on_date.to_string());
    //     }
    // }
    // else{}
    update_query.created_on_date = Set(user.created_on_date.clone());
    update_query.last_login=  Set(user.last_login.clone());
    let workspace_id=body.workspace_id.clone();
    if user.workspace_id!= workspace_id {
        changes.insert("user_id".to_string(), workspace_id.to_string());
        update_query.workspace_id = Set(user.workspace_id.clone());
    }
    let is_active=body.is_active.clone();
    if is_active!=user.is_active{
        update_query.is_active = Set(is_active.clone());
        changes.insert("is_active".to_string(), is_active.to_string());
    }
    
    
    let is_deleted=body.is_deleted.clone();        
    if user.is_deleted!= is_deleted {
        changes.insert("is_deleted".to_string(), is_deleted.to_string());
        update_query.is_deleted = Set(is_deleted.clone());
    }
    
    // let last_login=body.last_login.clone();
    // if!last_login.to_string().is_empty() {
    //     update_query.last_login = Set(last_login.clone());
    //     if user.last_login!= last_login {
    //         changes.insert("last_login".to_string(), last_login.to_string());
    //     }
    // }
    let password = body.password.clone();
    

    if !password.is_empty() {
        // Create a new hasher instance
        let mut hasher = Sha256::new();
    
        // Write input data (password) to the hasher
        hasher.update(password.clone());
    
        // Hash the password and convert to a hexadecimal string
        let hashed_password = format!("{:x}", hasher.finalize());
    
        // Update the query with the hashed password
        update_query.password = Set(hashed_password.clone());
        // Check if the existing password in the database is different from the new hashed password
        if user.password != hashed_password {
            // Add the change to the `changes` map
            changes.insert("password".to_string(), hashed_password);
        }
    }
    let is_admin=body.is_admin.clone();  
    if user.is_admin!= is_admin {
        update_query.is_admin= Set(is_admin.clone());
        changes.insert("is_admin".to_string(), is_admin.to_string());
    }
    


    (changes, update_query)
}
