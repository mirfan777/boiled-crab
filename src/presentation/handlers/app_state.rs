use std::sync::Arc;
use crate::application::services::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
}
