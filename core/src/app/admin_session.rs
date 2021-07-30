use crate::models::{AdminUser};
use async_trait::async_trait;

pub use crate::contracts::repo::SessionCreateError as RepoError;
use crate::contracts::UnexpectedDatabaseError;

#[async_trait]
pub trait AdminSession {
    async fn admin_session_token_get(&self, cookie: String) -> Result<Option<AdminUser>, AdminSessionResolveError>;
}

#[derive(Debug, thiserror::Error)]
pub enum AdminSessionResolveError {
    #[error("Unexpected admin session resolve failure: {0}")]
    Unexpected(#[from] eyre::Report),
    #[error("User is unauthorized")]
    Unauthorized
}

impl From<UnexpectedDatabaseError> for AdminSessionResolveError {
    #[inline]
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
