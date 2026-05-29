use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlPool;
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use crate::domain::DomainError;

pub struct MySqlUserRepository {
    pool: MySqlPool,
}

impl MySqlUserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn create(&self, user: &User) -> Result<User, DomainError> {
        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.id.to_string())
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(user.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let user = sqlx::query_as::<_, (String, String, String, DateTime<Utc>, DateTime<Utc>)>(
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.map(|(id, email, password_hash, created_at, updated_at)| User {
            id: Uuid::parse_str(&id).unwrap(),
            email,
            password_hash,
            created_at,
            updated_at,
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let user = sqlx::query_as::<_, (String, String, String, DateTime<Utc>, DateTime<Utc>)>(
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.map(|(id, email, password_hash, created_at, updated_at)| User {
            id: Uuid::parse_str(&id).unwrap(),
            email,
            password_hash,
            created_at,
            updated_at,
        }))
    }

    async fn update(&self, user: &User) -> Result<User, DomainError> {
        sqlx::query(
            r#"
            UPDATE users
            SET email = ?, password_hash = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(Utc::now())
        .bind(user.id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(user.clone())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, DomainError> {
        let users = sqlx::query_as::<_, (String, String, String, String, String)>(
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(users
            .into_iter()
            .map(|(id, email, password_hash, _created_at, _updated_at)| {
                User::new(Uuid::parse_str(&id).unwrap(), email, password_hash)
            })
            .collect())
    }
}
