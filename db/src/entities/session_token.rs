use crate::chrono::Utc;
use accesso_core::models;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub(crate) struct SessionToken {
    pub(crate) user_id: uuid::Uuid,
    pub(crate) token: String,
    pub(crate) expires_at: chrono::DateTime<Utc>,
}

impl From<models::SessionToken> for SessionToken {
    fn from(session: models::SessionToken) -> Self {
        Self {
            user_id: session.user_id,
            token: session.token,
            expires_at: session.expires_at,
        }
    }
}

impl Into<models::SessionToken> for SessionToken {
    fn into(self) -> models::SessionToken {
        models::SessionToken {
            user_id: self.user_id,
            token: self.token,
            expires_at: self.expires_at,
        }
    }
}
