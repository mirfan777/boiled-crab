use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use super::handlers::AppState;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(super::handlers::health))
        .route("/api/auth/register", post(super::handlers::register))
        .route("/api/auth/login", post(super::handlers::login))
        .route("/api/me", get(super::handlers::profile))
        .route("/api/users/{user_id}", get(super::handlers::get_user))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
