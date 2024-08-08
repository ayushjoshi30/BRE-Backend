use std::sync::Arc;
use entity::g_workspaces as tenants;
use sea_orm::{ DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use warp::{reject, reply::Reply};
use crate::error::Error::*;
use crate::WebResult;
use entity::g_workspaces::Entity as TenantEntity;


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

