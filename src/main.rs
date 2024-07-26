mod routes;
mod controllers {
    pub mod tenant_handler;
}
mod models {
    pub mod tenant_model;
}

#[tokio::main]
async fn main() {
    let routes = routes::routes();

    println!("Server started at http://localhost:9000");
    warp::serve(routes).run(([127, 0, 0, 1], 9000)).await;
}
