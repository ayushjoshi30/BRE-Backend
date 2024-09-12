use sea_orm::DatabaseConnection;
use warp:: {http::header::{HeaderMap, HeaderValue}, Filter};
use std::sync::Arc;
use crate::controllers::workspace_handler::*;
use crate::auth::auth::with_auth;
pub fn create_workspace(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create_workspace")
        .and(warp::post())
        .and(with_headers())
        .and(with_auth())
        .and(warp::body::json())
        .and(with_pool(db_pool))
        .and_then(create_workspace_handler)
}
// A Route to handle user
pub fn read_workspace(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_workspace")
        .and(warp::get())
        .and(with_auth())
        .and(with_pool(db_pool))
        .and_then(read_workspace_handler)
}
pub fn read_all_workspaces(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_all_workspaces")
       .and(warp::get())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(read_all_workspaces_handler)
}
pub fn update_workspace(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("update_workspace"/i32)
       .and(warp::put())
       .and(with_auth())
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(update_workspace_handler)
}

pub fn delete_workspace(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("delete_workspace"/i32)
        .and(warp::delete())
        .and(with_auth())
        .and(with_pool(db_pool))
        .and_then(delete_workspace_handler)
}
fn with_pool(db_pool: Arc<DatabaseConnection>) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
pub fn with_headers() -> impl Filter<Extract = (HeaderMap<HeaderValue>,), Error = std::convert::Infallible> + Clone {
    warp::header::headers_cloned()
}