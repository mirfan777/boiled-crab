use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::User;
use crate::domain::DomainError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn update(&self, user: &User) -> Result<User, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, DomainError>;
}
