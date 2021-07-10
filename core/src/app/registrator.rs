use crate::contracts::{RegisterUserError, SendEmailError};
use async_trait::async_trait;
use chrono::Utc;

#[async_trait]
pub trait Registrator {
    async fn registrator_create_request(
        &self,
        form: CreateRegisterRequest,
    ) -> Result<RequestCreated, RegisterRequestError>;

    async fn registrator_confirm(&self, form: RegisterForm) -> Result<(), RegisterConfirmError>;
}

#[derive(Debug, Clone, Validate)]
pub struct CreateRegisterRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Validate)]
pub struct RegisterForm {
    #[validate(length(min = 7))]
    pub confirmation_code: String,

    #[validate(length(min = 2))]
    pub first_name: String,

    #[validate(length(min = 2))]
    pub last_name: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug)]
pub struct RequestCreated {
    pub expires_at: chrono::DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum RegisterRequestError {
    #[error(transparent)]
    InvalidForm(#[from] validator::ValidationErrors),
    #[error("Email already registered {0}")]
    EmailAlreadyRegistered(String),
    #[error("Failed to send email: {0}")]
    EmailSenderError(#[from] SendEmailError),
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum RegisterConfirmError {
    #[error("{0}")]
    InvalidForm(#[from] validator::ValidationErrors),
    #[error("Code not found")]
    CodeNotFound,
    #[error("Code already activated: {0}")]
    AlreadyActivated(#[source] RegisterUserError),
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("Failed to send email: {0}")]
    EmailSenderError(#[from] SendEmailError),
}

impl From<RegisterUserError> for RegisterConfirmError {
    fn from(e: RegisterUserError) -> Self {
        match e {
            RegisterUserError::EmailAlreadyExists => Self::AlreadyActivated(e),
            _ => Self::Unexpected(e.into()),
        }
    }
}

impl CreateRegisterRequest {
    pub fn from_email<E>(email: E) -> Self
    where
        E: Into<String>,
    {
        Self {
            email: email.into(),
        }
    }
}
