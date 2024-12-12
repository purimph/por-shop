use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sea_orm::Database;
use std::env;

mod controllers;
mod entity;
mod routes;
mod services;
mod error;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // อ่านค่าจาก environment variable
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Error: DATABASE_URL must be set");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "DATABASE_URL not set",
            ));
        }
    };

    // พยายามเชื่อมต่อกับฐานข้อมูล
    let db = match Database::connect(&database_url).await {
        Ok(connection) => connection,
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database connection failed",
            ));
        }
    };

    // เริ่ม HTTP Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(routes::configure_auth)
            .configure(routes::configure_product)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
