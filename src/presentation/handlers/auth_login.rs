use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use validator::Validate;

use crate::application::dtos::LoginUserRequest;
use super::app_state::AppState;

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
