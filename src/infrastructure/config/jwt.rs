use std::env;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct JwtConfig {
	pub secret: String,
	pub expiration: u64,
	pub algorithm: Option<String>,
}

impl JwtConfig {
	pub fn from_env() -> Self {
		let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

		let secret = match env::var("JWT_SECRET") {
			Ok(s) => s,
			Err(_) => {
				if app_env == "production" {
					panic!("JWT_SECRET must be set in production environment");
				} else {
					warn!("JWT_SECRET not set, using insecure default for development");
					"secret".to_string()
				}
			}
		};

		let expiration = env::var("JWT_EXPIRATION")
			.unwrap_or_else(|_| "86400".to_string())
			.parse()
			.unwrap_or(86400);

		let algorithm = match env::var("JWT_ALGORITHM") {
			Ok(a) => Some(a),
			Err(_) => {
				if app_env == "production" {
					panic!("JWT_ALGORITHM must be set in production environment");
				} else {
					warn!("JWT_ALGORITHM not set for development; behavior will default to HS256 at runtime");
					None
				}
			}
		};

		Self { secret, expiration, algorithm }
	}
}
