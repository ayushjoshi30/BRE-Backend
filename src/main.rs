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
mod connection;
mod datafeeding{
    pub mod data_entry;
}
use std::sync::Arc;

use connection::init_db;
use controllers::login_handler::*;
use sea_orm::DatabaseConnection;
use warp::Filter;
use tokio::{signal, sync::{Mutex, OnceCell}};



#[tokio::main]
async fn main() {
     // Initialize the database connection asynchronously
     let db = init_db().await;

    // Create Arc Pool
    let db_pool = Arc::new(db);

    let routes = routes::routes(db_pool.clone()).recover(error::handle_rejection);
    
    // Create a future that listens for the shutdown signal
    let (_, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(([127,0,0,1], 9000), async {
            signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
            println!("Server Shutting down gracefully ...");
        });

    // Run the server and wait for the shutdown signal
    println!("Server started at http://127.0.0.1:9000");
    server.await;
}
