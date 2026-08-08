use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn init_db() -> DatabaseConnection {
    match Database::connect(env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file"))
        .await
    {
        Ok(db) => db,
        Err(e) => panic!("Error connecting to database: {}", e),
    }
}
