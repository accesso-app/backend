use crate::chrono::Utc;
use accesso_core::models;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub(crate) struct AuthorizationCode {
    pub(crate) client_id: uuid::Uuid,
    pub(crate) code: String,
    pub(crate) created_at: chrono::DateTime<Utc>,
    pub(crate) redirect_uri: String,
    pub(crate) scope: Option<Vec<String>>,
    pub(crate) user_id: uuid::Uuid,
}

impl From<models::AuthorizationCode> for AuthorizationCode {
    fn from(authorization_code: models::AuthorizationCode) -> Self {
        Self {
            client_id: authorization_code.client_id,
            code: authorization_code.code,
            created_at: authorization_code.created_at,
            redirect_uri: authorization_code.redirect_uri,
            scope: if authorization_code.scopes.is_empty() {
                None
            } else {
                Some(authorization_code.scopes)
            },
            user_id: authorization_code.user_id,
        }
    }
}

impl Into<models::AuthorizationCode> for AuthorizationCode {
    fn into(self) -> models::AuthorizationCode {
        models::AuthorizationCode {
            client_id: self.client_id,
            code: self.code,
            created_at: self.created_at,
            redirect_uri: self.redirect_uri,
            scopes: self.scope.unwrap_or_default(),
            user_id: self.user_id,
        }
    }
}
