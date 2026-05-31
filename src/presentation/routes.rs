use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::infrastructure::config::build_cors_layer;

use super::{handlers::AppState, middleware::{auth_middleware, auth_rate_limit_middleware}};

pub fn create_routes(state: AppState, app_env: &str) -> Router {
    let cors = build_cors_layer(app_env)
        .unwrap_or_else(|err| panic!("Failed to build CORS configuration: {}", err));

    let auth_routes = Router::new()
        .route("/register", post(super::handlers::register))
        .route("/login", post(super::handlers::login))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_rate_limit_middleware,
        ));

    let protected_routes = Router::new()
        .route("/me", get(super::handlers::profile))
        .route("/users/{user_id}", get(super::handlers::get_user))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/health", get(super::handlers::health))
        .nest("/api/auth", auth_routes)
        .nest("/api", protected_routes)
        .layer(cors)
        .with_state(state)
}
