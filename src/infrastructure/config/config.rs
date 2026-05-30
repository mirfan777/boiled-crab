use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub db_connection: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_database: String,
    pub db_username: String,
    pub db_password: String,
    pub app_host: String,
    pub app_port: u16,
    pub app_env: String,
    pub jwt_secret: String,
    pub jwt_expiration: u64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            db_connection: env::var("DB_CONNECTION").unwrap_or_else(|_| "mysql".to_string()),
            db_host: env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            db_port: env::var("DB_PORT")
                .unwrap_or_else(|_| "3306".to_string())
                .parse()
                .unwrap_or(3306),
            db_database: env::var("DB_DATABASE").unwrap_or_else(|_| "boiled_crab".to_string()),
            db_username: env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string()),
            db_password: env::var("DB_PASSWORD").unwrap_or_else(|_| String::new()),
            app_host: env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            app_port: env::var("APP_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            app_env: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string()),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .unwrap_or(86400),
        }
    }

    pub fn database_url(&self) -> String {
        if self.db_password.is_empty() {
            format!(
                "{}://{}@{}:{}/{}",
                self.db_connection, self.db_username, self.db_host, self.db_port, self.db_database
            )
        } else {
            format!(
                "{}://{}:{}@{}:{}/{}",
                self.db_connection,
                self.db_username,
                self.db_password,
                self.db_host,
                self.db_port,
                self.db_database
            )
        }
    }
}
