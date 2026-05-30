use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: Uuid, name: String, email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            email,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }
}
