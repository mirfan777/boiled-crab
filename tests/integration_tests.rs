use std::sync::Arc;

use axum::{body::{Body, to_bytes}, http::{Request, StatusCode}};
use boiled_crab::{
    application::services::AuthService,
    domain::{entities::User, repositories::UserRepository, DomainError},
    presentation::{handlers::AppState, routes::create_routes, middleware::AuthRateLimiter},
};
use bcrypt::{hash, DEFAULT_COST};
use jsonwebtoken::Algorithm;
use mockall::{mock, predicate::*};
use serde_json::{json, Value};
use tower::ServiceExt;
use uuid::Uuid;

mock! {
    UserRepository {}

    #[async_trait::async_trait]
    impl UserRepository for UserRepository {
        async fn create(&self, user: &User) -> Result<User, DomainError>;
        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
        async fn update(&self, user: &User) -> Result<User, DomainError>;
        async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
        async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, DomainError>;
    }
}

fn test_user(user_id: Uuid) -> User {
    User::new(
        user_id,
        "Test User".to_string(),
        "test@example.com".to_string(),
        hash("password123", DEFAULT_COST).expect("hash password"),
    )
}

fn build_state(mock_repo: Arc<dyn UserRepository>) -> AppState {
    let auth_service = Arc::new(AuthService::new(
        mock_repo,
        "test-secret-key-test-secret-key".to_string(),
        3600,
        Algorithm::HS256,
    ));

    AppState {
        auth_service,
        rate_limiter: Arc::new(AuthRateLimiter::new(10, std::time::Duration::from_secs(60))),
    }
}

async fn response_json(response: axum::response::Response) -> Value {
    let bytes = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("read body");

    serde_json::from_slice(&bytes).expect("valid json")
}

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let state = build_state(Arc::new(MockUserRepository::new()));
    let app = create_routes(state, "development");

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).expect("request"))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response_json(response).await;
    assert_eq!(body, json!({"status": "ok"}));
}

#[tokio::test]
async fn protected_route_requires_bearer_token() {
    let state = build_state(Arc::new(MockUserRepository::new()));
    let app = create_routes(state, "development");

    let response = app
        .oneshot(Request::builder().uri("/api/me").body(Body::empty()).expect("request"))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn profile_endpoint_returns_current_user() {
    let user_id = Uuid::new_v4();
    let user = test_user(user_id);

    let mut mock_repo = MockUserRepository::new();
    mock_repo
        .expect_find_by_id()
        .with(eq(user_id))
        .returning(move |_| Ok(Some(user.clone())));

    let state = build_state(Arc::new(mock_repo));
    let token = state
        .auth_service
        .generate_token(&user_id)
        .expect("generate token");

    let app = create_routes(state, "development");
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/me")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response_json(response).await;
    assert_eq!(body["data"]["id"], json!(user_id));
    assert_eq!(body["data"]["email"], json!("test@example.com"));
}

#[tokio::test]
async fn register_endpoint_rejects_invalid_payload() {
    let state = build_state(Arc::new(MockUserRepository::new()));
    let app = create_routes(state, "development");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(json!({"email": "invalid"}).to_string()))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
