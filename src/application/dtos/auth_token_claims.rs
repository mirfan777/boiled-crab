use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokenClaims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
}
