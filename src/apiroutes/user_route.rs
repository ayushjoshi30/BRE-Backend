use sea_orm::DatabaseConnection;
use warp:: {http::header::{HeaderMap, HeaderValue}, Filter};
use std::sync::Arc;
use crate::controllers::user_handler::*;
use crate::auth::auth::with_auth;
pub fn create_user(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("create_user")
       .and(warp::post())
       .and(with_auth())
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(create_user_handler)
}
// Read User
pub fn read_user(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_user" / i32)
       .and(warp::get())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(read_user_handler)
}

// Read All Users
pub fn read_all_users(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_all_users")
       .and(warp::get())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(read_all_users_handler)
}
pub fn update_user(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("update_user" / i32)
       .and(warp::put())
       .and(with_auth())
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(update_user_handler)
}
// Delete User
pub fn delete_user(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("delete_user" / u32)
       .and(warp::delete())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(delete_user_handler)
}
fn with_pool(db_pool: Arc<DatabaseConnection>) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}