// Kernel implementation (kept here) -- removed incorrect re-export
use sea_orm::Database;
use std::{sync::Arc, time::Duration};
use tracing::info;

use crate::infrastructure::config::Config;
use crate::infrastructure::services::create_auth_service;
use crate::presentation::handlers::AppState;
use crate::presentation::middleware::AuthRateLimiter;
use crate::presentation::routes::create_routes;

pub struct Kernel;

impl Kernel {
    pub async fn bootstrap() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize tracing and other global platform pieces
        dotenv::dotenv().ok();
        tracing_subscriber::fmt::init();
        Ok(())
    }

    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        // 1. Load config
        let config = Config::from_env();
        info!("Configuration loaded: env={}", config.app_env);

        // 2. Database
        let db = Database::connect(&config.database_url()).await?;
        info!("Database connected successfully");

        // 3. Services / DI
        let auth_service = create_auth_service(&config, db);
        let rate_limiter = Arc::new(AuthRateLimiter::new(5, Duration::from_secs(300)));
        let state = AppState {
            auth_service,
            rate_limiter,
        };

        // 4. Router
        let app = create_routes(state, &config.app_env);

        // 5. Listener & Serve with graceful shutdown
        let addr = format!("{}:{}", config.app_host, config.app_port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        info!("Server listening on {}", addr);

        axum::serve(listener, app)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await?;

        Ok(())
    }

    async fn shutdown_signal() {
        // Wait for Ctrl+C
        if let Err(e) = tokio::signal::ctrl_c().await {
            tracing::error!("failed to listen for shutdown signal: {}", e);
        }
        info!("Shutting down application gracefully...");
    }
}
