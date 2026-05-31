use std::sync::Arc;
use sea_orm::DatabaseConnection;

use crate::application::services::AuthService;
use crate::infrastructure::database::SeaOrmUserRepository;
use crate::infrastructure::config::Config;
use jsonwebtoken::Algorithm;
use crate::domain::repositories::UserRepository;

pub fn create_auth_service(config: &Config, db: DatabaseConnection) -> Arc<AuthService> {
    // Force Rust to coerce the concrete repository into a trait object using `as`.
    let user_repo = Arc::new(SeaOrmUserRepository::new(db)) as Arc<dyn UserRepository>;
    // parse algorithm string -> Algorithm enum
    let alg = match config.auth.jwt.algorithm.as_deref() {
        Some("HS256") => Algorithm::HS256,
        Some("HS384") => Algorithm::HS384,
        Some("HS512") => Algorithm::HS512,
        Some(other) => {
            tracing::warn!("Unsupported JWT algorithm '{}', defaulting to HS256", other);
            Algorithm::HS256
        }
        None => {
            tracing::info!("No JWT_ALGORITHM configured (development); defaulting to HS256 at runtime");
            Algorithm::HS256
        }
    };

    Arc::new(AuthService::new(
        user_repo,
        config.auth.jwt.secret.clone(),
        config.auth.jwt.expiration,
        alg,
    ))
}