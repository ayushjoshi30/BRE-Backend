use sea_orm::DatabaseConnection;
use warp::Filter;
use std::sync::Arc;
use crate::controllers::login_handler::*;
use crate::controllers::user_handler::*;
use crate::controllers::rule_handler::*;
use crate::controllers::workspace_handler::*;
use crate::controllers::audittrail_handler::*;
use crate::controllers::release_handler::*;
use crate::controllers::configure_handler::*;
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
    let release_routes = warp::path("release")
        .and(
            create_release(db_pool.clone())
                .or(read_release(db_pool.clone()))
                .or(read_all_releases(db_pool.clone()))
                .or(update_release(db_pool.clone()))
                .or(delete_release(db_pool.clone()))
        );
    let audittrail_routes = warp::path("audittrail")
        .and(
            create_audittrail(db_pool.clone())
                .or(read_audittrail(db_pool.clone()))
                .or(read_all_audittrails(db_pool.clone()))
                .or(update_audittrail(db_pool.clone()))
                .or(delete_audittrail(db_pool.clone()))
        );
    let configure_routes = warp::path("configure")
        .and(
            create_configure(db_pool.clone())
                .or(read_configure(db_pool.clone()))
                .or(read_all_configures(db_pool.clone()))
                .or(update_configure(db_pool.clone()))
                .or(delete_configure(db_pool.clone()))
        );
    login_route(db_pool.clone())
        .or(user_routes)
        .or(workspace_routes)
        .or(rule_routes)
        .or(release_routes)
        .or(audittrail_routes)
        .or(configure_routes)
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
    warp::path!("read_rule")
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
    warp::path!("delete_rule" / i32)
       .and(warp::delete())
       .and(with_auth())
       .and(with_pool(db_pool))
       .and_then(delete_rule_handler)
}
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