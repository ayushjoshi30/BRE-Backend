// use warp::Filter;
use crate::models::tenant_model::Tenant;

// A function to handle GET requests at /posts/{id}
pub async fn get_tenant(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post
    let tenant_obj = Tenant {
        id,
        tenant: String::from("Universal"),
        authkey: String::from("ajcdbsakjcbsjkcbsdu223="),
    };
    Ok(warp::reply::json(&tenant_obj))
}