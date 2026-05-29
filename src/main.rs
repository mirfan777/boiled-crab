mod domain;
mod application;
mod infrastructure;
mod presentation;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

use application::services::AuthService;
use infrastructure::config::Config;
use infrastructure::database::MySqlUserRepository;
use presentation::handlers::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::from_env();
    info!("Configuration loaded");

    // Initialize database connection pool
    let database_url = config.database_url();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    info!("Database connected successfully");

    // Create repository
    let user_repository = Arc::new(MySqlUserRepository::new(pool));

    // Create services
    let auth_service = Arc::new(AuthService::new(
        user_repository,
        config.jwt_secret.clone(),
        config.jwt_expiration,
    ));

    // Create application state
    let state = AppState { auth_service };

    // Build router
    let app = Router::new()
        .route("/health", get(presentation::handlers::health))
        .route("/api/auth/register", post(presentation::handlers::register))
        .route("/api/auth/login", post(presentation::handlers::login))
        .route("/api/users/{user_id}", get(presentation::handlers::get_user))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Create listener
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.app_host, config.app_port))
        .await?;

    info!(
        "Server listening on {}:{}",
        config.app_host, config.app_port
    );

    axum::serve(listener, app).await?;

    Ok(())
}
