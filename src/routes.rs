use sea_orm::DatabaseConnection;
use warp::Filter;
use std::sync::Arc;
use crate::controllers::login_handler::*;
use crate::apiroutes::user_route::*;
use crate::apiroutes::workspace_route::*;
use crate::apiroutes::rule_route::*;
use crate::apiroutes::release_route::*;
use crate::apiroutes::audittrail_route::*;
use crate::apiroutes::configure_route::*;
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
                .or(save_draft(db_pool.clone()))
                .or(publish_rule(db_pool.clone()))
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
fn with_pool(db_pool: Arc<DatabaseConnection>) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}