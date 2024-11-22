mod auth {
    pub mod auth;
}
mod apiroutes{
    pub mod user_route;
    pub mod workspace_route;
    pub mod rule_route;
    pub mod release_route;
    pub mod audittrail_route;
    pub mod configure_route;
}
mod error;
mod models {
    pub mod login_model;
    pub mod workspace_model;
    pub mod rules_model;
    pub mod release_model;
}
mod controllers {
    pub mod login_handler;
    pub mod user_handler;
    pub mod workspace_handler;
    pub mod rule_handler;
    pub mod release_handler;
    pub mod audittrail_handler;
    pub mod configure_handler;
    pub mod zen_handler;
}
mod routes;
// mod util;
mod connection;
use std::sync::Arc;
use connection::init_db;
use controllers::login_handler::*;
use warp::Filter;
use tokio::signal;
use log::{info, warn};
use log4rs;



#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default())
    .expect("Failed to initialize logger. Ensure that the log4rs.yaml file exists and is in the correct location.");

    // // Log the startup process
    info!("Initializing database connection...");
     // Initialize the database connection asynchronously
     let db = init_db().await;

    // Create Arc Pool
    let db_pool = Arc::new(db);
    info!("Setting up routes...");
    let routes = routes::routes(db_pool.clone()).recover(error::handle_rejection);
    info!("Starting server at http://127.0.0.1:9000");
    // Create a future that listens for the shutdown signal
    let (_, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(([127,0,0,1], 9000), async {
            signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
            warn!("Server shutting down gracefully...");
            // println!("Server Shutting down gracefully ...");
        });

    // Run the server and wait for the shutdown signal
    println!("Server started at http://127.0.0.1:9000");
    server.await;
    info!("Server has shut down.");
}
