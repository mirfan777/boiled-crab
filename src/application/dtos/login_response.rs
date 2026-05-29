use serde::{Deserialize, Serialize};
use crate::application::dtos::UserResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}
