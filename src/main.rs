mod domain;
mod application;
mod infrastructure;
mod presentation;

use sea_orm::Database;
use std::sync::Arc;
use tracing::info;

use application::services::AuthService;
use infrastructure::config::Config;
use infrastructure::database::SeaOrmUserRepository;
use presentation::handlers::AppState;
use presentation::routes::create_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::from_env();
    info!("Configuration loaded");

    // Initialize database connection
    let database_url = config.database_url();
    let db = Database::connect(&database_url).await?;

    info!("Database connected successfully");

    // Create repository
    let user_repository = Arc::new(SeaOrmUserRepository::new(db));

    // Create services
    let auth_service = Arc::new(AuthService::new(
        user_repository,
        config.jwt_secret.clone(),
        config.jwt_expiration,
    ));

    // Create application state
    let state = AppState { auth_service };

    // Build router
    let app = create_routes(state);

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
