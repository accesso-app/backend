use accesso_core::models;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UserRegistration {
    pub(crate) id: uuid::Uuid,
    pub(crate) client_id: uuid::Uuid,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) user_id: uuid::Uuid,
}

impl From<models::UserRegistration> for UserRegistration {
    fn from(reg: models::UserRegistration) -> Self {
        Self {
            id: reg.id,
            client_id: reg.client_id,
            created_at: reg.created_at,
            user_id: reg.user_id,
        }
    }
}

impl Into<models::UserRegistration> for UserRegistration {
    fn into(self) -> models::UserRegistration {
        models::UserRegistration {
            id: self.id,
            client_id: self.client_id,
            created_at: self.created_at,
            user_id: self.user_id,
        }
    }
}
