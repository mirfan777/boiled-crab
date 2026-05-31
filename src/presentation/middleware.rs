use std::{collections::HashMap, sync::Arc, time::{Duration, Instant}};

use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tokio::sync::Mutex;

use crate::domain::DomainError;

use super::handlers::AppState;

#[derive(Clone)]
pub struct AuthRateLimiter {
    inner: Arc<Mutex<HashMap<String, RateLimitWindow>>>,
    max_attempts: u32,
    window: Duration,
}

#[derive(Debug, Clone)]
struct RateLimitWindow {
    started_at: Instant,
    attempts: u32,
}

impl AuthRateLimiter {
    pub fn new(max_attempts: u32, window: Duration) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            max_attempts,
            window,
        }
    }

    pub async fn allow(&self, key: &str) -> bool {
        let mut store = self.inner.lock().await;
        let now = Instant::now();

        match store.get_mut(key) {
            Some(window) if now.duration_since(window.started_at) <= self.window => {
                if window.attempts >= self.max_attempts {
                    false
                } else {
                    window.attempts += 1;
                    true
                }
            }
            Some(window) => {
                *window = RateLimitWindow {
                    started_at: now,
                    attempts: 1,
                };
                true
            }
            None => {
                store.insert(
                    key.to_string(),
                    RateLimitWindow {
                        started_at: now,
                        attempts: 1,
                    },
                );
                true
            }
        }
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let token = match extract_bearer_token(request.headers()) {
        Some(token) => token,
        None => return unauthorized_response("Missing or invalid authorization header"),
    };

    match state.auth_service.verify_token(token) {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            next.run(request).await
        }
        Err(DomainError::UnauthorizedError(message)) => unauthorized_response(&message),
        Err(err) => {
            tracing::error!("Authentication middleware error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Authentication failed"})),
            )
                .into_response()
        }
    }
}

pub async fn auth_rate_limit_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let key = rate_limit_key(request.headers());

    if !state.rate_limiter.allow(&key).await {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({"error": "Too many authentication attempts. Please try again later."})),
        )
            .into_response();
    }

    next.run(request).await
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|token| !token.is_empty())
}

fn rate_limit_key(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|value| value.to_str().ok())
                .map(str::trim)
                .filter(|value| !value.is_empty())
        })
        .unwrap_or("global")
        .to_string()
}

fn unauthorized_response(message: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": message})),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::AuthRateLimiter;
    use std::time::Duration;

    #[tokio::test]
    async fn rate_limiter_blocks_after_limit() {
        let limiter = AuthRateLimiter::new(2, Duration::from_secs(60));

        assert!(limiter.allow("127.0.0.1").await);
        assert!(limiter.allow("127.0.0.1").await);
        assert!(!limiter.allow("127.0.0.1").await);
    }
}
