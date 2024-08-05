use sea_orm::DatabaseConnection;
use warp::Filter;
use crate::models::login_model::*;
use std::sync::Arc;
use crate::controllers::login_handler::*;
use crate::auth::auth::{with_auth, Role};

// A function to build our routes
pub fn routes(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Make db available to all routes
    login_route()
        .or(user_route(db_pool.clone()))
        .or(admin_route(db_pool.clone()))
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
pub fn user_route(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(with_auth(Role::User))
        .and(with_pool(db_pool))
        .and_then(user_handler)
}

// A Route to handle admin
pub fn admin_route(db_pool : Arc<DatabaseConnection>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("admin")
        .and(with_auth(Role::Admin))
        .and(with_pool(db_pool))
        .and_then(admin_handler)
}

fn with_pool(db_pool: Arc<DatabaseConnection>) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}