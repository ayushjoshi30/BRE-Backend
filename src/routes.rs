use warp::Filter;
use crate::models::login_model::*;
use std::sync::Arc;
use crate::controllers::login_handler::*;
use crate::auth::auth::{with_auth, Role};

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    login_route()
        .or(user_route())
        .or(admin_route())
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
pub fn user_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler)
}

// A Route to handle admin
pub fn admin_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("admin")
        .and(with_auth(Role::Admin))
        .and_then(admin_handler)
}