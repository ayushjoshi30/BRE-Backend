use sea_orm::DatabaseConnection;
use warp::Filter;
use std::sync::Arc;
use crate::controllers::audittrail_handler::*;
use crate::auth::auth::with_auth;

pub fn create_audittrail(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create_audittrail")
        .and(warp::post())
        .and(with_auth())
        .and(warp::body::json())
        .and(with_pool(db_pool))
        .and_then(create_audittrail_handler)
}
pub fn read_audittrail(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_audittrail"/i32)
        .and(warp::get())
        .and(with_auth())
        .and(with_pool(db_pool))
        .and_then(read_audittrail_handler)
}
pub fn read_all_audittrails(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_all_audittrails")
       .and(warp::get())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(read_all_audittrail_handler)
}
pub fn update_audittrail(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("update_audittrail" / i32)
       .and(warp::put())
       .and(with_auth())
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(update_audittrail_handler)
}
pub fn delete_audittrail(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("delete_audittrail" / i32)
       .and(warp::delete())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(delete_audittrail_handler)
}
fn with_pool(db_pool: Arc<DatabaseConnection>) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}