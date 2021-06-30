use crate::contracts::repo::UnexpectedDatabaseError;
use crate::models::{SessionToken, User};

pub use crate::contracts::repo::SessionCreateError as RepoError;

pub trait Session {
    fn session_resolve_by_cookie(
        &self,
        cookie: String,
    ) -> Result<Option<User>, SessionResolveError>;

    fn session_resolve_by_access_token(
        &self,
        access_token: String,
    ) -> Result<Option<User>, SessionResolveError>;

    fn session_create(
        &self,
        form: SessionCreateForm,
    ) -> Result<(SessionToken, User), SessionCreateError>;

    fn session_delete(
        &self,
        user: &User,
        strategy: SessionDeleteStrategy,
    ) -> Result<(), SessionDeleteError>;
}

pub enum SessionDeleteStrategy {
    All,
    Single(String),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SessionResolveError {
    Unexpected,
}

#[derive(Debug, Validate, PartialEq, Eq, Hash)]
pub struct SessionCreateForm {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SessionCreateError {
    Unexpected,
    InvalidForm,
    InvalidCredentials,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SessionDeleteError {
    Unexpected,
}

impl From<validator::ValidationErrors> for SessionCreateError {
    fn from(_: validator::ValidationErrors) -> Self {
        Self::InvalidForm
    }
}

impl From<UnexpectedDatabaseError> for SessionCreateError {
    fn from(_: UnexpectedDatabaseError) -> Self {
        Self::Unexpected
    }
}
impl From<RepoError> for SessionCreateError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::Unexpected | RepoError::TokenAlreadyExists => Self::Unexpected,
            RepoError::UserNotFound => Self::InvalidCredentials,
        }
    }
}
