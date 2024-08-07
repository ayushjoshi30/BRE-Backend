use sea_orm::DatabaseConnection;
use warp::Filter;
use crate::models::login_model::*;
use std::sync::Arc;
use crate::controllers::login_handler::*;
use crate::controllers::user_handler::*;
use crate::controllers::tenant_handler::*;
use crate::auth::auth::{with_auth, Role};

// A function to build our routes
pub fn routes(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Make db available to all routes
    login_route()
        .or(create_user(db_pool.clone()))
        .or(read_user(db_pool.clone()))
        .or(create_tenant(db_pool.clone()))
        .or(read_tenant(db_pool.clone()))
        .or(delete_tenant(db_pool.clone()))
        .or(update_tenant(db_pool.clone()))
        .or(read_all_users(db_pool.clone()))
        .or(read_all_tenants(db_pool.clone()))
        .or(update_user(db_pool.clone()))
        .or(delete_user(db_pool.clone()))
}

// A Route to handle login
pub fn login_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let users = Arc::new(init_users());

    warp::path!("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(login_handler)
}

// A Route to handle user

pub fn read_tenant(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_tenant"/i32)
        .and(warp::get())
        .and(with_auth(Role::Admin))
        .and(with_pool(db_pool))
        .and_then(read_tenant_handler)
}
pub fn create_user(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("create_user")
       .and(warp::post())
       .and(with_auth(Role::Admin))
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(create_user_handler)
}
// Read User
pub fn read_user(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_user" / i32)
       .and(warp::get())
       .and(with_auth(Role::Admin))
       .and(with_pool(db_pool))
       .and_then(read_user_handler)
}

pub fn create_tenant(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("create_tenant")
       .and(warp::post())
       .and(with_auth(Role::Admin))
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(create_tenant_handler)
}

pub fn delete_tenant(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("delete_tenant"/i32)
        .and(warp::delete())
        .and(with_auth(Role::Admin))
        .and(with_pool(db_pool))
        .and_then(delete_tenant_handler)
}
pub fn update_tenant(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("update_tenant"/i32)
       .and(warp::put())
       .and(with_auth(Role::Admin))
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(update_tenant_handler)
}
// Read All Users
pub fn read_all_users(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_all_users")
       .and(warp::get())
       .and(with_auth(Role::Admin))
       .and(with_pool(db_pool))
       .and_then(read_all_users_handler)
}pub fn read_all_tenants(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("read_all_tenants")
       .and(warp::get())
       .and(with_auth(Role::Admin))
       .and(with_pool(db_pool))
       .and_then(read_all_tenants_handler)
}
// Update User
pub fn update_user(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("update_user" / u32)
       .and(warp::put())
       .and(with_auth(Role::Admin))
       .and(warp::body::json())
       .and(with_pool(db_pool))
       .and_then(update_user_handler)
}

// Delete User
pub fn delete_user(db_pool : Arc<DatabaseConnection>)->impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("delete_user" / u32)
       .and(warp::delete())
       .and(with_auth(Role::Admin))
       .and(with_pool(db_pool))
       .and_then(delete_user_handler)
}

fn with_pool(db_pool: Arc<DatabaseConnection>) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}