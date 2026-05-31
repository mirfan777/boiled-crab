use std::sync::Arc;

use crate::application::services::AuthService;
use crate::presentation::middleware::AuthRateLimiter;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub rate_limiter: Arc<AuthRateLimiter>,
}
