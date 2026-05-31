use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use validator::Validate;
use uuid::Uuid;

use crate::application::dtos::{AuthTokenClaims, LoginUserRequest, RegisterUserRequest};
use super::AppState;

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginUserRequest>,
) -> Response {
    if let Err(_) = req.validate() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Validation failed"}))).into_response();
    }

    match state.auth_service.login(req).await {
        Ok(response) => {
            tracing::info!("User logged in successfully");
            (StatusCode::OK, Json(json!({"data": response}))).into_response()
        }
        Err(err) => {
            tracing::error!("Login error: {}", err);
            err.into_response()
        }
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterUserRequest>,
) -> Response {
    if let Err(_) = req.validate() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Validation failed"}))).into_response();
    }

    match state.auth_service.register(req).await {
        Ok(user) => {
            tracing::info!("User registered successfully: {}", user.email);
            (StatusCode::CREATED, Json(json!({"data": user}))).into_response()
        }
        Err(err) => {
            tracing::error!("Register error: {}", err);
            err.into_response()
        }
    }
}

pub async fn profile(
    State(state): State<AppState>,
    Extension(claims): Extension<AuthTokenClaims>,
) -> Response {
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(user_id) => user_id,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid token subject"})),
            )
                .into_response();
        }
    };

    match state.auth_service.get_user(user_id).await {
        Ok(user) => {
            tracing::info!("Profile fetched for user: {}", user.email);
            (StatusCode::OK, Json(json!({"data": user}))).into_response()
        }
        Err(err) => {
            tracing::error!("Profile error: {}", err);
            err.into_response()
        }
    }
}
