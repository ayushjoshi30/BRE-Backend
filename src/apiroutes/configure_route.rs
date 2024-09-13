use sea_orm::DatabaseConnection;
use warp::Filter;
use std::sync::Arc;
use crate::controllers::configure_handler::*;
use crate::auth::auth::with_auth;
pub fn create_configure(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create_configure")
        .and(warp::post())
        .and(with_auth())
        .and(warp::body::json())
        .and(with_pool(db_pool))
        .and_then(create_configure_handler)
}
pub fn read_configure(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_configure"/i32)
        .and(warp::get())
        .and(with_auth())
        .and(with_pool(db_pool))
        .and_then(read_configure_handler)
}
pub fn read_all_configures(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_all_configures")
       .and(warp::get())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(read_all_configure_handler)
}
pub fn update_configure(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("update_configure" / i32)
       .and(warp::put())
       .and(with_auth())
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(update_configure_handler)
}
pub fn delete_configure(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("delete_configure" / i32)
       .and(warp::delete())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(delete_configure_handler)
}
fn with_pool(db_pool: Arc<DatabaseConnection>) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}