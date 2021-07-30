use accesso_core::models;
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct AdminSessionToken {
    pub(crate) user_id: uuid::Uuid,
    pub(crate) token: String,
    pub(crate) expires_at: DateTime<Utc>,
}

impl From<AdminSessionToken> for models::AdminSessionToken {
    fn from(token: AdminSessionToken) -> Self {
        Self {
            user_id: token.user_id,
            token: token.token,
            expires_at: token.expires_at,
        }
    }
}

impl From<models::AdminSessionToken> for AdminSessionToken {
    fn from(token: models::AdminSessionToken) -> Self {
        Self {
            user_id: token.user_id,
            token: token.token,
            expires_at: token.expires_at,
        }
    }
}
