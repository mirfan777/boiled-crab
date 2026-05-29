use axum::{
    extract::{State, Path},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use super::app_state::AppState;

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Response {
    match state.auth_service.get_user(user_id).await {
        Ok(user) => Json(json!({"data": user})).into_response(),
        Err(err) => err.into_response(),
    }
}
