use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use std::sync::Arc;

use crate::application::{
    dtos::{RegisterUserRequest, LoginUserRequest},
    services::AuthService,
};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterUserRequest>,
) -> Response {
    if let Err(_) = req.validate() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Validation failed"}))).into_response();
    }

    match state.auth_service.register(req).await {
        Ok(user) => (StatusCode::CREATED, Json(json!({"data": user}))).into_response(),
        Err(err) => err.into_response(),
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginUserRequest>,
) -> Response {
    if let Err(_) = req.validate() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Validation failed"}))).into_response();
    }

    match state.auth_service.login(req).await {
        Ok(response) => (StatusCode::OK, Json(json!({"data": response}))).into_response(),
        Err(err) => err.into_response(),
    }
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Response {
    match state.auth_service.get_user(user_id).await {
        Ok(user) => Json(json!({"data": user})).into_response(),
        Err(err) => err.into_response(),
    }
}

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}
