use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::application::dtos::AuthTokenClaims;

use super::AppState;

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(claims): Extension<AuthTokenClaims>,
) -> Response {
    let authenticated_user_id = match Uuid::parse_str(&claims.sub) {
        Ok(user_id) => user_id,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid token subject"})),
            )
                .into_response();
        }
    };

    if authenticated_user_id != user_id {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "You are not allowed to access this resource"})),
        )
            .into_response();
    }

    match state.auth_service.get_user(user_id).await {
        Ok(user) => Json(json!({"data": user})).into_response(),
        Err(err) => err.into_response(),
    }
}
