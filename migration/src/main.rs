use sea_orm_migration::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv::dotenv().ok();
    
    // Build DATABASE_URL from individual env vars
    let db_connection = env::var("DB_CONNECTION").unwrap_or_else(|_| "mysql".to_string());
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "3306".to_string());
    let db_database = env::var("DB_DATABASE").unwrap_or_else(|_| "boiled".to_string());
    let db_username = env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| String::new());
    
    let database_url = if db_password.is_empty() {
        format!(
            "{}://{}@{}:{}/{}",
            db_connection, db_username, db_host, db_port, db_database
        )
    } else {
        format!(
            "{}://{}:{}@{}:{}/{}",
            db_connection, db_username, db_password, db_host, db_port, db_database
        )
    };
    
    env::set_var("DATABASE_URL", &database_url);
    println!("DATABASE_URL set to: {}", database_url);
    
    cli::run_cli(migration::Migrator).await;
}
