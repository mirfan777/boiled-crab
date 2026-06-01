# Improvements

## Checked

- Environment-aware CORS implemented (development permissive, production requires `APP_ALLOWED_WEB`).
- Fail-fast secret loading implemented for `JWT_SECRET` and `JWT_ALGORITHM` in production.
- Extracted `JwtConfig` / `AuthConfig` and moved overall config into `config/app.rs`.
- Moved `AuthService` wiring into an infrastructure factory (`create_auth_service`).
- Implemented real authentication middleware for protected routes using `AuthService::verify_token()`.
- Added rate limiting for login and registration endpoints.
- Added a Kernel/app bootstrap to keep `main.rs` thin.
- Coerced concrete repository into `Arc<dyn UserRepository>` to fix trait-object mismatch.
- Codebase compiles (`cargo check`) after the refactors (warnings only).

## Must Improve (remaining / high priority)

- Replace any remaining `unwrap()` calls (notably `Uuid::parse_str(...).unwrap()` in repository mapping).
- Harden JWT validation (check `iss`, `aud`, `alg`, expiration, and clock skew handling).
- Validate and parse `APP_ALLOWED_WEB` on startup; fail-fast if missing in production and tighten origin parsing.
- Improve error handling and logging; avoid leaking DB or parsing internals in HTTP responses.
- Expand unit and integration tests for services, repositories, and auth flows.
- Remove unused or placeholder code and align the README with project reality.
- Decide and document whether this repository is a SeaORM-based app or a reusable framework; make dependency choices consistent.
- Add input normalization and stricter validation for names, emails, and passwords before persisting.
- Add observability basics: structured logs, request IDs, and tracing.
- Apply security-focused defaults for production: secure secret loading, env checks, and safe host binding.

**Clean Architecture & Clean Code**

- Enforce layer boundaries: keep `domain` free of framework code, `application` contains use-cases, `infrastructure` holds adapters (DB, config), and `presentation` contains handlers/HTTP. Audit imports to ensure dependency rule holds.
- Use traits as ports: keep repository interfaces in `src/domain/repositories.rs` and implement adapters in `src/infrastructure/database/*` (avoid referencing infra from domain).
- Thin composition root: keep wiring in `src/infrastructure/kernel.rs` or a single `bootstrap` module; avoid application wiring in `main.rs` or handlers.
- Single Responsibility: split large modules into focused files (e.g., move handler helpers into `presentation/handlers/*`, keep `AuthService` focused on auth use-cases).
- Explicit types & validation: use domain types for IDs/Email/Password where possible and validate at boundaries (DTOs -> `application` -> `domain`).
- Error discipline: use domain error types (`src/domain/errors.rs`) and map external errors at adapter boundaries; avoid panics and `unwrap()`.
- Dependency injection: pass dependencies via constructors or `AppState`; avoid globals and singletons.
- Naming & module hygiene: avoid duplicate/confusing names (e.g., `services.rs` in multiple folders) — prefer `application/services/auth.rs`, `infrastructure/auth_factory.rs`.
- Tests per layer: add unit tests for domain and application logic, adapter tests for repositories (use test DB or mocks), and integration tests for end-to-end flows.
- Documentation & examples: add a short `docs/ARCHITECTURE.md` or expand `README.md` showing the intended layering, extension points, and where to add new adapters.

Suggested files to inspect/update:

- `src/domain/*` — ensure no axum/sea-orm imports
- `src/application/services/*` — keep pure business logic, no HTTP
- `src/infrastructure/*` and `src/infrastructure/database/*` — adapters and factories
- `src/presentation/*` — HTTP bindings, middleware, and request/response mapping

Small next step I can apply: scan imports to find infra imports inside `domain` and list them. Want me to do that? 

If you want, I can start with the highest-priority item (replace `unwrap()` uses in repositories). 

## Examples — files to update and suggested snippets

- Replace `unwrap()` usages in repository mapping (`src/infrastructure/database/repository/user.rs`):

```rust
// current (unsafe)
id: Uuid::parse_str(&model.id).unwrap(),

// suggested: propagate parse errors as DomainError
let id = Uuid::parse_str(&model.id)
	.map_err(|e| DomainError::InternalError(format!("invalid uuid in DB: {}", e)))?;

let user = User { id, name: model.name, email: model.email, password_hash: model.password_hash, created_at: model.created_at, updated_at: model.updated_at };
```

- Fail-fast secret loading (`src/infrastructure/config/jwt.rs` / `src/infrastructure/config/app.rs`):

```rust
let jwt_secret = std::env::var("JWT_SECRET")
	.map_err(|_| anyhow::anyhow!("JWT_SECRET must be set in production"))?;
```

- Auth middleware skeleton (`src/presentation/middleware.rs`):

```rust
pub async fn auth_middleware<B>(req: Request<B>, next: Next<B>, state: AppState) -> impl IntoResponse {
	let header = req.headers().get("Authorization");
	let token = extract_bearer(header).ok_or(StatusCode::UNAUTHORIZED)?;
	let claims = state.auth_service.verify_token(&token)?;
	req.extensions_mut().insert(claims);
	next.run(req).await
}
```

- CORS origins parsing and validation (`src/infrastructure/config/cors.rs`):

```rust
let allowed = std::env::var("APP_ALLOWED_WEB").unwrap_or_default();
if app_env == "production" && allowed.trim().is_empty() {
	return Err(anyhow::anyhow!("APP_ALLOWED_WEB required in production"));
}
let origins: Vec<_> = allowed.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
```

- Rate limiter placement: add a middleware or route-layer in `src/presentation/routes.rs` to wrap `POST /login` and `POST /register`.

- Tests: add unit tests under `src/application/services` and integration tests under `tests/` covering register/login/token flows.

These examples are minimal — I can open PR-style patches to implement any of them. Which one should I start with? 
