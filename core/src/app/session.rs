use crate::contracts::repo::{
    GetUserBySessionError, SessionCreateError as RepoError, SessionRepo, UnexpectedDatabaseError,
    UserCredentials, UserRepo,
};
use crate::contracts::secure::SecureGenerator;
use crate::models::{SessionToken, User};
use crate::App;
use validator::Validate;

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
        &mut self,
        form: SessionCreateForm,
    ) -> Result<(SessionToken, User), SessionCreateError>;
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

const MAX_TOKEN_CREATE_ATTEMPTS: u8 = 10;
const SESSION_TOKEN_LIVE_DAYS: u8 = 14;

impl<DB, E, G> Session for App<DB, E, G>
where
    DB: SessionRepo + UserRepo,
    G: SecureGenerator,
{
    fn session_resolve_by_cookie(
        &self,
        cookie: String,
    ) -> Result<Option<User>, SessionResolveError> {
        match self.db.get_user_by_session_token(cookie) {
            Err(GetUserBySessionError::Unexpected) => Err(SessionResolveError::Unexpected),
            Err(GetUserBySessionError::NotFound) => Ok(None),
            Ok(user) => Ok(Some(user)),
        }
    }

    fn session_resolve_by_access_token(
        &self,
        access_token: String,
    ) -> Result<Option<User>, SessionResolveError> {
        match self.db.get_user_by_access_token(access_token) {
            Err(GetUserBySessionError::Unexpected) => Err(SessionResolveError::Unexpected),
            Err(GetUserBySessionError::NotFound) => Ok(None),
            Ok(user) => Ok(Some(user)),
        }
    }

    fn session_create(
        &mut self,
        form: SessionCreateForm,
    ) -> Result<(SessionToken, User), SessionCreateError> {
        form.validate()?;

        let hashed_input_password = self.generator.password_hash(form.password.clone());

        let found_user = self.db.user_find_by_credentials(UserCredentials {
            email: form.email.clone(),
            password_hash: hashed_input_password,
        })?;

        if let Some(user) = found_user {
            let mut insert_attempt = 0u8;

            let session: SessionToken = loop {
                insert_attempt += 1;

                let token = self.generator.generate_token();
                let result = self.db.session_create(SessionToken {
                    user_id: user.id.clone(),
                    token,
                    expires_at: chrono::Utc::now().naive_utc()
                        + chrono::Duration::days(SESSION_TOKEN_LIVE_DAYS as i64),
                });

                if let Err(RepoError::TokenAlreadyExists) = result {
                    if insert_attempt <= MAX_TOKEN_CREATE_ATTEMPTS {
                        continue;
                    }
                }

                break result;
            }?;

            Ok((session, user))
        } else {
            Err(SessionCreateError::InvalidCredentials)
        }
    }
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
