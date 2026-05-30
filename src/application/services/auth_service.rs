use crate::application::dtos::{RegisterUserRequest, LoginUserRequest, UserResponse, LoginResponse, AuthTokenClaims};
use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use crate::domain::DomainError;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use uuid::Uuid;
use std::sync::Arc;

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    jwt_secret: String,
    jwt_expiration: u64,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        jwt_secret: String,
        jwt_expiration: u64,
    ) -> Self {
        Self {
            user_repository,
            jwt_secret,
            jwt_expiration,
        }
    }

    pub async fn register(&self, req: RegisterUserRequest) -> Result<UserResponse, DomainError> {
        // Check if user already exists
        if let Ok(Some(_)) = self.user_repository.find_by_email(&req.email).await {
            return Err(DomainError::ConflictError("Email already exists".to_string()));
        }

        // Hash password
        let password_hash = hash(req.password, DEFAULT_COST)
            .map_err(|_| DomainError::InternalError("Failed to hash password".to_string()))?;

        // Create user
        let user = User::new(Uuid::new_v4(), req.name, req.email, password_hash);
        let created_user = self.user_repository.create(&user).await?;

        Ok(UserResponse {
            id: created_user.id,
            name: created_user.name,
            email: created_user.email,
            created_at: created_user.created_at.to_rfc3339(),
            updated_at: created_user.updated_at.to_rfc3339(),
        })
    }

    pub async fn login(&self, req: LoginUserRequest) -> Result<LoginResponse, DomainError> {
        // Find user by email
        let user = self
            .user_repository
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| DomainError::UnauthorizedError("Invalid email or password".to_string()))?;

        // Verify password
        let password_valid = verify(&req.password, &user.password_hash)
            .map_err(|_| DomainError::UnauthorizedError("Invalid email or password".to_string()))?;

        if !password_valid {
            return Err(DomainError::UnauthorizedError("Invalid email or password".to_string()));
        }

        // Generate JWT token
        let token = self.generate_token(&user.id)?;

        Ok(LoginResponse {
            token,
            user: UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                created_at: user.created_at.to_rfc3339(),
                updated_at: user.updated_at.to_rfc3339(),
            },
        })
    }

    pub fn generate_token(&self, user_id: &Uuid) -> Result<String, DomainError> {
        let now = Utc::now();
        let exp = (now.timestamp() as u64) + self.jwt_expiration;

        let claims = AuthTokenClaims {
            sub: user_id.to_string(),
            exp,
            iat: now.timestamp() as u64,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| DomainError::InternalError("Failed to generate token".to_string()))
    }

    pub fn verify_token(&self, token: &str) -> Result<AuthTokenClaims, DomainError> {
        decode(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| DomainError::UnauthorizedError("Invalid token".to_string()))
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<UserResponse, DomainError> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".to_string()))?;

        Ok(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        UserRepository {}
        
        #[async_trait]
        impl UserRepository for UserRepository {
            async fn create(&self, user: &User) -> Result<User, DomainError>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
            async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
            async fn update(&self, user: &User) -> Result<User, DomainError>;
            async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
            async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, DomainError>;
        }
    }

    #[tokio::test]
    async fn test_register_user_success() {
        let mut mock_repo = MockUserRepository::new();
        
        mock_repo
            .expect_find_by_email()
            .with(eq("test@example.com"))
            .return_once(|_| Ok(None));

        mock_repo
            .expect_create()
            .returning(|user| Ok(user.clone()));

        let auth_service = AuthService::new(
            Arc::new(mock_repo),
            "secret".to_string(),
            3600,
        );

        let req = RegisterUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = auth_service.register(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_user_email_exists() {
        let mut mock_repo = MockUserRepository::new();
        
        let existing_user = User::new(
            Uuid::new_v4(),
            "Test User".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
        );

        mock_repo
            .expect_find_by_email()
            .with(eq("test@example.com"))
            .return_once(|_| Ok(Some(existing_user)));

        let auth_service = AuthService::new(
            Arc::new(mock_repo),
            "secret".to_string(),
            3600,
        );

        let req = RegisterUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = auth_service.register(req).await;
        assert!(matches!(result, Err(DomainError::ConflictError(_))));
    }

    #[tokio::test]
    async fn test_login_user_success() {
        let mut mock_repo = MockUserRepository::new();
        
        let user_id = Uuid::new_v4();
        let password = "password123";
        let password_hash = hash(password, DEFAULT_COST).unwrap();
        
        let user = User::new(
            user_id,
            "Test User".to_string(),
            "test@example.com".to_string(),
            password_hash,
        );

        mock_repo
            .expect_find_by_email()
            .with(eq("test@example.com"))
            .return_once(move |_| Ok(Some(user.clone())));

        let auth_service = AuthService::new(
            Arc::new(mock_repo),
            "secret".to_string(),
            3600,
        );

        let req = LoginUserRequest {
            email: "test@example.com".to_string(),
            password: password.to_string(),
        };

        let result = auth_service.login(req).await;
        assert!(result.is_ok());
    }
}
