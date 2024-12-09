use sea_orm::{Database, DatabaseConnection};
use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpServer};
// use std::sync::Arc;

mod routes;
mod controllers;
mod models;
mod services;
// mod schemas;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to database
    let db: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    // Start Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Share database connection
            .configure(routes::configure)        // Configure routes
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}