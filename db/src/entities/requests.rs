use crate::chrono::Utc;
use accesso_core::models;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub(crate) struct RegistrationRequest {
    pub(crate) confirmation_code: String,
    pub(crate) email: String,
    pub(crate) expires_at: chrono::DateTime<Utc>,
}

impl From<models::RegisterRequest> for RegistrationRequest {
    fn from(model: models::RegisterRequest) -> Self {
        Self {
            confirmation_code: model.code,
            email: model.email,
            expires_at: model.expires_at,
        }
    }
}

impl Into<models::RegisterRequest> for RegistrationRequest {
    fn into(self) -> models::RegisterRequest {
        models::RegisterRequest {
            code: self.confirmation_code,
            email: self.email,
            expires_at: self.expires_at,
        }
    }
}
