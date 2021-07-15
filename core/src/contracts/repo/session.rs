use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;

use crate::contracts::UnexpectedDatabaseError;
use crate::models::{SessionToken, User};

#[derive(Debug, thiserror::Error)]
pub enum GetUserBySessionError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("Not found")]
    NotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionCreateError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("Token already exists")]
    TokenAlreadyExists,
    #[error("User not found")]
    UserNotFound,
}

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait SessionRepo {
    async fn get_user_by_session_token(&self, token: String)
        -> Result<User, GetUserBySessionError>;
    async fn get_user_by_access_token(&self, token: String) -> Result<User, GetUserBySessionError>;
    async fn session_create(
        &self,
        session: SessionToken,
    ) -> Result<SessionToken, SessionCreateError>;
    async fn session_delete_token(
        &self,
        session_token: &str,
    ) -> Result<(), UnexpectedDatabaseError>;
    async fn session_delete_by_user_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<(), UnexpectedDatabaseError>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl SessionRepo for crate::contracts::MockDb {
    async fn get_user_by_session_token(
        &self,
        token: String,
    ) -> Result<User, GetUserBySessionError> {
        self.session.get_user_by_session_token(token).await
    }
    async fn get_user_by_access_token(&self, token: String) -> Result<User, GetUserBySessionError> {
        self.session.get_user_by_access_token(token).await
    }
    async fn session_create(
        &self,
        session: SessionToken,
    ) -> Result<SessionToken, SessionCreateError> {
        self.session.session_create(session).await
    }

    async fn session_delete_token(
        &self,
        session_token: &str,
    ) -> Result<(), UnexpectedDatabaseError> {
        self.session.session_delete_token(session_token).await
    }
    async fn session_delete_by_user_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<(), UnexpectedDatabaseError> {
        self.session.session_delete_by_user_id(user_id).await
    }
}
