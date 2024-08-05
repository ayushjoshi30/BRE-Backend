use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use dotenv::dotenv;
use std::env;

pub async fn init_db() -> DatabaseConnection {
    dotenv().ok();
    let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Run the database
    let mut opt = ConnectOptions::new(format!("{}", DATABASE_URL));
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        // .sqlx_logging_level(log::Level::Debug)
        .set_schema_search_path("public"); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await.unwrap();

    // Ping the database
    match db.ping().await {
        Ok(_) => println!("Database is up and running"),
        Err(DbErr::Custom(e)) => {
            eprintln!("Database error: {:?}", e);
            // Raise Panic
            panic!("Database error");
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            // Raise Panic
            panic!("Database error");
        }
    }

    return db;
}