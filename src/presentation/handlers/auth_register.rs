use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use validator::Validate;

use crate::application::dtos::RegisterUserRequest;
use super::app_state::AppState;

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
