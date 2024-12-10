// use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sea_orm::{Database, DbErr};
use std::env;
// use std::sync::Arc;
#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Attempting to connect to: {}", database_url);

    match Database::connect(&database_url).await {
        Ok(_) => {
            println!(
                "Connected leaw jaa!");
            Ok(())
        }
        Err(e) => {
            eprintln!(
                "Connection Bor Dai: {:?}",e);
            Err(e)
        }
    }
}
