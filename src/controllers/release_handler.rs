use std::sync::Arc;
use entity::g_releases as releases;
use std::collections::HashMap;
use serde_json::json;
use sea_orm::{ ActiveModelTrait,DatabaseConnection, EntityTrait, QueryFilter,Set, ColumnTrait};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_releases::Entity as ReleaseEntity;
pub async fn create_release_handler(authenticated: String ,body: releases::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{

    print!("Request Authenticated: {}", authenticated);

    let release = releases::ActiveModel {
        version: Set(body.version),
        file_path: Set(body.file_path),
        file_json:Set(body.file_json),
        created_at: Set(body.created_at),
        // Set the last login to the current time

        is_released: Set(body.is_released),
        released_date:Set(body.released_date),
        created_by_user: Set(body.created_by_user),
        // Set the last login to the current time
        ..Default::default()
    };

    let release: releases::Model = release.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert release: {:?}", e);
        reject::custom(InvalidRequestBodyError)
    })?;

    Ok(warp::reply::json(&release))
}

pub async fn read_release_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match ReleaseEntity::find()
    .filter(releases::Column::Id.eq(id))
    .one(&*db_pool)
    .await
    {
    // If the release is found, return it as JSON
    Ok(Some(release)) => Ok(warp::reply::json(&release)),

    // If the release is not found, return a 404 error
    Ok(None) => Err(reject::custom(ResourceNotFound)),

    // If there is a database error, return a generic database error
    Err(_) => Err(reject::custom(DatabaseError)),
    }
}
pub async fn update_release_handler(id:i32,_:String,body: releases::Model,db_pool:Arc<DatabaseConnection>)->WebResult<impl Reply>{
    let release = ReleaseEntity::find().filter(releases::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;
    
    let release = release.ok_or(reject::custom(ResourceNotFound))?;
    let (changes, release_model)  = update_map_releases(release.clone(), body.clone(), id);
    let updated_release = release_model.update(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    // Construct a response with the changes made
    let response = json!({
        "message": "release updated successfully",
        "changes": changes,
        "entity": updated_release
    });

    Ok(warp::reply::json(&response))
}
pub async fn delete_release_handler(id:i32,_:String,db_pool:Arc<DatabaseConnection>) -> WebResult<impl Reply>{
    match releases::Entity::delete_many()
        .filter(releases::Column::Id.eq(id.clone()))
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
pub async fn read_all_release_handler(_:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match ReleaseEntity::find().all(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(releases) => Ok(warp::reply::json(&releases)),
        Err(_) => Err(reject::custom(DatabaseError)),
    }
}

fn update_map_releases(release: releases::Model, body: releases::Model, id: i32) -> (HashMap<String, String>, releases::ActiveModel) {
    let mut update_query = releases::ActiveModel {
        id: Set(id),
        ..Default::default() // Start with default values
    };

    let mut changes = HashMap::new();

    let version = body.version.clone();
    if !version.is_empty() {
        update_query.version = Set(version.clone());
        if release.version != version {
            changes.insert("version".to_string(), version);
        }
    }
    // Handle `file_path`
    let file_path = body.file_path.clone();
    if !file_path.is_empty() {
        update_query.file_path = Set(file_path.clone());
        if release.file_path != file_path {
            changes.insert("file_path".to_string(), file_path);
        }
    }
    let file_json= body.file_json.clone();
    println!("{:?}",file_json);
    println!("{:?}",release.file_json);

    if release.file_json!= file_json {
        update_query.file_json = Set(file_json.clone());
        changes.insert("file_json".to_string(), file_json.to_string());
    }
    let created_at=body.created_at.clone();
    if release.created_at!= created_at {
        update_query.created_at = Set(created_at.clone());
        changes.insert("created_at".to_string(), created_at.to_string());
    }
    let is_released=body.is_released.clone();
    if release.is_released!= is_released {
        update_query.is_released = Set(is_released.clone());
        changes.insert("is_released".to_string(), is_released.to_string());
    }
    
    let released_date=body.released_date.clone();
    if release.released_date!= released_date {
        changes.insert("user_id".to_string(), released_date.to_string());
        update_query.released_date = Set(release.released_date.clone());
    }
    let created_by_user=body.created_by_user.clone();      
    if created_by_user != 0 && release.created_by_user != created_by_user {
        changes.insert("created_by_user".to_string(), created_by_user.to_string());
        update_query.created_by_user = Set(created_by_user);
    }
    (changes, update_query)
}

