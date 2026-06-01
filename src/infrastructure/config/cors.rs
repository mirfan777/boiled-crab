use axum::http::{header, HeaderValue, Method, Uri};
use tower_http::cors::CorsLayer;

pub fn build_cors_layer(app_env: &str) -> Result<CorsLayer, String> {
	match app_env.to_ascii_lowercase().as_str() {
		"development" => Ok(CorsLayer::permissive()),
		"production" | "prod" => build_production_cors_layer(),
		unknown => Err(format!(
			"APP_ENV '{}' is not recognized. Use 'development' or 'production'",
			unknown
		)),
	}
}

fn build_production_cors_layer() -> Result<CorsLayer, String> {
	let allowed_origins = [
		"https://app.example.com",
		"https://admin.example.com",
	];

	let origins: Vec<HeaderValue> = allowed_origins
		.iter()
		.copied()
		.map(|origin| {
			validate_origin(origin)?;
			HeaderValue::from_str(origin)
				.map_err(|_| format!("Invalid hardcoded origin value: {}", origin))
		})
		.collect::<Result<Vec<_>, _>>()?;

	if origins.is_empty() {
		return Err("Hardcoded allowed origins list cannot be empty in production mode".to_string());
	}

	Ok(CorsLayer::new()
		.allow_origin(origins)
		.allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
		.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]))
}

fn validate_origin(origin: &str) -> Result<(), String> {
	let uri: Uri = origin
		.parse()
		.map_err(|_| format!("Origin must use a valid format such as https://example.com: {}", origin))?;

	if uri.scheme().is_none() || uri.authority().is_none() {
		return Err(format!(
			"Origin must include a scheme and host, for example https://example.com: {}",
			origin
		));
	}

	if uri.path() != "/" && !uri.path().is_empty() {
		return Err(format!(
			"Origin cannot include a path: {}",
			origin
		));
	}

	if uri.query().is_some() {
		return Err(format!(
			"Origin cannot include a query string: {}",
			origin
		));
	}

	match uri.scheme_str() {
		Some("http") | Some("https") => Ok(()),
		Some(other) => Err(format!("Unsupported origin scheme '{}': {}", other, origin)),
		None => Err(format!("Origin must include a scheme: {}", origin)),
	}
}

#[cfg(test)]
mod tests {
	use super::validate_origin;

	#[test]
	fn accepts_valid_http_origin() {
		assert!(validate_origin("http://localhost:5173").is_ok());
	}

	#[test]
	fn rejects_origin_with_path() {
		assert!(validate_origin("https://example.com/api").is_err());
	}

	#[test]
	fn rejects_invalid_scheme() {
		assert!(validate_origin("ftp://example.com").is_err());
	}
}
