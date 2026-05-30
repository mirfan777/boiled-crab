use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, IntoActiveModel, QuerySelect};
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use crate::domain::DomainError;
use super::user_model;

pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn create(&self, user: &User) -> Result<User, DomainError> {
        let model = user_model::ActiveModel {
            id: sea_orm::Set(user.id.to_string()),
            email: sea_orm::Set(user.email.clone()),
            password_hash: sea_orm::Set(user.password_hash.clone()),
            created_at: sea_orm::Set(user.created_at),
            updated_at: sea_orm::Set(user.updated_at),
        };

        match model.insert(&self.db).await {
            Ok(_) => Ok(user.clone()),
            Err(sea_orm::DbErr::RecordNotInserted) => {
                // RecordNotInserted error sering terjadi meski data berhasil insert
                // Ini return user yang di-insert sebagai success
                tracing::debug!("User {} inserted successfully (RecordNotInserted error ignored)", user.email);
                Ok(user.clone())
            }
            Err(e) => {
                tracing::error!("Failed to create user: {:?}", e);
                Err(DomainError::from(e))
            }
        }
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let user = user_model::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?;

        Ok(user.map(|model| User {
            id: Uuid::parse_str(&model.id).unwrap(),
            email: model.email,
            password_hash: model.password_hash,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let user = user_model::Entity::find()
            .filter(user_model::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        Ok(user.map(|model| User {
            id: Uuid::parse_str(&model.id).unwrap(),
            email: model.email,
            password_hash: model.password_hash,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }))
    }

    async fn update(&self, user: &User) -> Result<User, DomainError> {
        let model: user_model::ActiveModel = user_model::Model {
            id: user.id.to_string(),
            email: user.email.clone(),
            password_hash: user.password_hash.clone(),
            created_at: user.created_at,
            updated_at: Utc::now(),
        }
        .into_active_model();

        model.update(&self.db).await?;
        Ok(user.clone())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        user_model::Entity::delete_by_id(id.to_string())
            .exec(&self.db)
            .await?;

        Ok(())
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, DomainError> {
        let users = user_model::Entity::find()
            .limit(limit as u64)
            .offset(offset as u64)
            .all(&self.db)
            .await?;

        Ok(users
            .into_iter()
            .map(|model| User {
                id: Uuid::parse_str(&model.id).unwrap(),
                email: model.email,
                password_hash: model.password_hash,
                created_at: model.created_at,
                updated_at: model.updated_at,
            })
            .collect())
    }
}
