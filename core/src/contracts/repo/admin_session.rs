use crate::models::{AdminAccessToken, AdminSession};
use crate::contracts::GetUserBySessionError;
use async_trait::async_trait;
use uuid::Uuid;


#[derive(Debug, thiserror::Error)]
pub enum SomeError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("Default")]
    DEFAULT,
}

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait AdminSessionRepo {
    async fn get_admin_session_token(&self, token: String)
                                     -> Result<AdminSession, GetUserBySessionError>;
    async fn get_access_token(&self, user_id: Uuid)
                                             -> Result<AdminAccessToken, GetUserBySessionError>;
}
