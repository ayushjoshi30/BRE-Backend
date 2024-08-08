use std::sync::Arc;
use entity::g_appusers;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use serde_json::json;
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_appusers::Entity as UserEntity;



pub async fn read_user_handler(id: i32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().filter(g_appusers::Column::Id.eq(id)).one(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(Some(user)) => Ok(warp::reply::json(&user)),
        Ok(None) => Err(reject::custom(ResourceNotFound)),

        Err(_) => Err(reject::custom(DatabaseError)),
    }
}

pub async fn read_all_users_handler(_:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().all(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(g_appusers) => Ok(warp::reply::json(&g_appusers)),
        Err(_) => Err(reject::custom(DatabaseError)),
    }
}


pub async fn delete_user_handler(id: u32, _:String, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    let user = UserEntity::find().filter(g_appusers::Column::Id.eq(id)).one(&*db_pool).await.map_err(|_| reject::custom(DatabaseError))?;

    let user = user.ok_or(reject::custom(ResourceNotFound))?;

    let user = g_appusers::ActiveModel {
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
