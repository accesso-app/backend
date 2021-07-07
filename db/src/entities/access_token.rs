use crate::chrono::Utc;
use accesso_core::models;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub(crate) struct AccessToken {
    pub(crate) client_id: uuid::Uuid,
    pub(crate) token: String,
    pub(crate) user_id: uuid::Uuid,
    pub(crate) scopes: Vec<String>,
    pub(crate) expires_at: chrono::DateTime<Utc>,
}

impl From<models::AccessToken> for AccessToken {
    fn from(token: models::AccessToken) -> Self {
        Self {
            client_id: token.client_id,
            token: token.token,
            user_id: token.user_id,
            scopes: token.scopes,
            expires_at: token.expires_at,
        }
    }
}

impl Into<models::AccessToken> for AccessToken {
    fn into(self) -> models::AccessToken {
        models::AccessToken {
            client_id: self.client_id,
            token: self.token,
            user_id: self.user_id,
            scopes: self.scopes,
            expires_at: self.expires_at,
        }
    }
}
