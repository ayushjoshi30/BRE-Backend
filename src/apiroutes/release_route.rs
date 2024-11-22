use sea_orm::DatabaseConnection;
use warp::Filter;
use std::sync::Arc;
use crate::controllers::release_handler::*;
use crate::auth::auth::with_auth;
pub fn create_release(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create_release")
        .and(warp::post())
        .and(with_auth())
        .and(warp::body::json())
        .and(with_pool(db_pool))
        .and_then(create_release_handler)
}
pub fn read_release(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_release"/i32)
        .and(warp::get())
        .and(with_auth())
        .and(with_pool(db_pool))
        .and_then(read_release_handler)
}
pub fn read_release_version(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_release_version"/String)
        .and(warp::get())
        .and(with_auth())
        .and(with_pool(db_pool))
        .and_then(read_release_version_handler)
}
pub fn read_all_releases(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_all_releases")
       .and(warp::get())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(read_all_release_handler)
}
pub fn update_release(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("update_release" / i32)
       .and(warp::put())
       .and(with_auth())
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(update_release_handler)
}
pub fn delete_release(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("delete_release" / i32)
       .and(warp::delete())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(delete_release_handler)
}
pub fn ready_for_release(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("ready_for_release")
       .and(warp::post())
       .and(with_auth())
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(ready_for_release_handler)
}
pub fn publish_release(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("publishrelease")
       .and(warp::post())
       .and(warp::body::json())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(publish_release_handler)
}
fn with_pool(db_pool: Arc<DatabaseConnection>) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}