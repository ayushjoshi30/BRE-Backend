mod auth {
    pub mod auth;
}
mod error;
mod models {
    pub mod login_model;
}
mod controllers {
    pub mod login_handler;
}
mod routes;
// mod util;
use controllers::login_handler::*;
use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = routes::routes().recover(error::handle_rejection);
    println!("Server started at http://localhost:9000");
    warp::serve(routes).run(([127, 0, 0, 1], 9000)).await;
}
