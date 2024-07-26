use warp::Filter;
use crate::controllers::tenant_handler as handlers;

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_tenant()
}

// A route to handle GET requests for a specific post
fn get_tenant() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tenants" / u64)
        .and(warp::get())
        .and_then(handlers::get_tenant)
}