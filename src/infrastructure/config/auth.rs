use super::JwtConfig;

#[derive(Debug, Clone)]
pub struct AuthConfig {
	pub jwt: JwtConfig,
}

impl AuthConfig {
	pub fn from_env() -> Self {
		Self { jwt: JwtConfig::from_env() }
	}
}
