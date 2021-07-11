use crate::chrono::Utc;
use accesso_core::models;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub(crate) struct AccessToken {
    pub(crate) token: String,
    pub(crate) scopes: Vec<String>,
    pub(crate) expires_at: chrono::DateTime<Utc>,
    pub(crate) registration_id: uuid::Uuid,
}

impl From<models::AccessToken> for AccessToken {
    fn from(token: models::AccessToken) -> Self {
        Self {
            token: token.token,
            scopes: token.scopes,
            expires_at: token.expires_at,
            registration_id: token.registration_id,
        }
    }
}

impl Into<models::AccessToken> for AccessToken {
    fn into(self) -> models::AccessToken {
        models::AccessToken {
            token: self.token,
            scopes: self.scopes,
            expires_at: self.expires_at,
            registration_id: self.registration_id,
        }
    }
}
