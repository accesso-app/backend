use crate::contracts::repo::UnexpectedDatabaseError;
use crate::models::{SessionToken, User};
use async_trait::async_trait;

pub use crate::contracts::repo::SessionCreateError as RepoError;

#[async_trait]
pub trait Session {
    async fn session_resolve_by_cookie(
        &self,
        cookie: String,
    ) -> Result<Option<User>, SessionResolveError>;

    async fn session_resolve_by_access_token(
        &self,
        access_token: String,
    ) -> Result<Option<User>, SessionResolveError>;

    async fn session_create(
        &self,
        form: SessionCreateForm,
    ) -> Result<(SessionToken, User), SessionCreateError>;

    async fn session_delete(
        &self,
        user: &User,
        strategy: SessionDeleteStrategy,
    ) -> Result<(), SessionDeleteError>;
}

pub enum SessionDeleteStrategy {
    All,
    Single(String),
}

#[derive(Debug, thiserror::Error)]
pub enum SessionResolveError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, Validate, PartialEq, Eq, Hash)]
pub struct SessionCreateForm {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionCreateError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("Invalid form: {0}")]
    InvalidForm(#[from] validator::ValidationErrors),
    #[error("Invalid credentials")]
    InvalidCredentials,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionDeleteError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

impl From<UnexpectedDatabaseError> for SessionDeleteError {
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}

impl From<UnexpectedDatabaseError> for SessionCreateError {
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
impl From<RepoError> for SessionCreateError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::Unexpected(_) | RepoError::TokenAlreadyExists => {
                Self::Unexpected(eyre::eyre!("Unexpected or token already exists"))
            }
            RepoError::UserNotFound => Self::InvalidCredentials,
        }
    }
}
