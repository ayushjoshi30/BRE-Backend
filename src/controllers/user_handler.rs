use std::sync::Arc;

use chrono::NaiveDateTime;
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::users::Entity as UserEntity;


pub async fn create_user_handler(authenticated: bool ,body: users::Model,db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply>{

    print!("Request Authenticated: {}", authenticated);

    let null_date_time = NaiveDateTime::from_timestamp(0, 0);

    let user = users::ActiveModel {
        username: Set(body.username),
        workspace_id: Set(body.workspace_id),
        password: Set(body.password),
        role: Set(body.role),
        last_login: Set(null_date_time), // Set the last login to the current time
        ..Default::default()
    };

    let user: users::Model = user.insert(&*db_pool).await.map_err(|e| {
        eprintln!("Failed to insert user: {:?}", e);
        reject::custom(WrongCredentialsError)
    })?;

    Ok(warp::reply::json(&user))
}


pub async fn read_user_handler(id: u32, _:bool, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    match UserEntity::find().filter(users::Column::Id.eq(id)).one(&*db_pool).await {
        // If the user is empty, return a 404
        Ok(Some(user)) => Ok(warp::reply::json(&user)),
        Ok(None) => Err(reject::custom(ResourceNotFound)),

        Err(_) => Err(reject::custom(DatabaseErrorr)),
    }
}