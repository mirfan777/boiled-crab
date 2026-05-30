use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use validator::Validate;
use uuid::Uuid;

use crate::application::dtos::{LoginUserRequest, RegisterUserRequest};
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
    headers: HeaderMap,
) -> Response {
    // Extract Bearer token from Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if let Some(token) = auth_header {
        match state.auth_service.verify_token(token) {
            Ok(claims) => {
                if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
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
                } else {
                    (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid user ID"}))).into_response()
                }
            }
            Err(err) => {
                tracing::error!("Token verification error: {}", err);
                err.into_response()
            }
        }
    } else {
        (StatusCode::UNAUTHORIZED, Json(json!({"error": "Missing or invalid authorization header"}))).into_response()
    }
}
