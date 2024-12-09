use sea_orm:: {Database, DatabaseConnection, DbErr};
use dotenv::dotenv;
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(database_url).await
}