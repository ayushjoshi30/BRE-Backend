use sea_orm::DatabaseConnection;
use warp::Filter;
use std::sync::Arc;
use crate::controllers::login_handler::*;
use crate::controllers::user_handler::*;
use crate::controllers::rule_handler::*;
use crate::controllers::workspace_handler::*;
use crate::auth::auth::with_auth;

// A function to build our routes
pub fn routes(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    
 
    let workspace_routes = warp::path("workspace")
        .and(
            create_workspace(db_pool.clone())
                .or(read_workspace(db_pool.clone()))
                .or(read_all_workspaces(db_pool.clone()))
                .or(update_workspace(db_pool.clone()))
                .or(delete_workspace(db_pool.clone()))
                
        );
    let user_routes = warp::path("user")
        .and(
            create_user(db_pool.clone())
                .or(read_user(db_pool.clone()))
                .or(read_all_users(db_pool.clone()))
                .or(update_user(db_pool.clone()))
                .or(delete_user(db_pool.clone()))
        );
    let rule_routes = warp::path("rule")
        .and(
            create_rule(db_pool.clone())
                .or(read_rule(db_pool.clone()))
                .or(read_all_rules(db_pool.clone()))
                .or(update_rule(db_pool.clone()))
                .or(delete_rule(db_pool.clone()))
        );
 
    login_route(db_pool.clone())
        .or(user_routes)
        .or(workspace_routes)
        .or(rule_routes)
}

// A Route to handle login
pub fn login_route(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(db_pool))
        .and_then(login_handler)
}

pub fn create_workspace(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create_workspace")
        .and(warp::post())
        .and(with_auth())
        .and(warp::body::json())
        .and(with_pool(db_pool))
        .and_then(create_workspace_handler)
}
// A Route to handle user
pub fn read_workspace(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_workspace"/i32)
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
pub fn create_rule(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("create_rule")
       .and(warp::post())
       .and(with_auth())
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(create_rule_handler)
}
pub fn read_rule(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_rule"/i32)
        .and(warp::get())
        .and(with_auth())
        .and(with_pool(db_pool))
        .and_then(read_rule_handler)
}
pub fn read_all_rules(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_all_rules")
       .and(warp::get())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(read_all_rule_handler)
}
pub fn update_rule(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("update_rule" / i32)
       .and(warp::put())
       .and(with_auth())
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(update_rule_handler)
}
pub fn delete_rule(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("delete_rule" / u32)
       .and(warp::delete())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(delete_rule_handler)
}
fn with_pool(db_pool: Arc<DatabaseConnection>) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}