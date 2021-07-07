use crate::contracts::{RegisterUserError, SaveRegisterRequestError, UnexpectedDatabaseError};
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

#[derive(Debug, Clone, Validate, PartialEq, Eq, Hash)]
pub struct CreateRegisterRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Validate, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RequestCreated {
    pub expires_at: chrono::DateTime<Utc>,
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RegisterRequestError {
    Unexpected,
    InvalidForm,
    EmailAlreadyRegistered,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RegisterConfirmError {
    Unexpected,
    InvalidForm,
    CodeNotFound,
    AlreadyActivated,
}

impl From<UnexpectedDatabaseError> for RegisterRequestError {
    fn from(_: UnexpectedDatabaseError) -> Self {
        RegisterRequestError::Unexpected
    }
}

impl From<UnexpectedDatabaseError> for RegisterConfirmError {
    fn from(_: UnexpectedDatabaseError) -> Self {
        RegisterConfirmError::Unexpected
    }
}

impl From<RegisterUserError> for RegisterConfirmError {
    fn from(error: RegisterUserError) -> Self {
        match error {
            RegisterUserError::Unexpected => Self::Unexpected,
            RegisterUserError::EmailAlreadyExists => Self::AlreadyActivated,
        }
    }
}

impl From<validator::ValidationErrors> for RegisterConfirmError {
    fn from(_: validator::ValidationErrors) -> Self {
        Self::InvalidForm
    }
}

impl From<SaveRegisterRequestError> for RegisterRequestError {
    fn from(_: SaveRegisterRequestError) -> Self {
        // Now all errors from request errors converted to Unexpected
        // Because CodeAlreadyExists is handled at create_register_request impl
        Self::Unexpected
    }
}

impl From<validator::ValidationErrors> for RegisterRequestError {
    fn from(_: validator::ValidationErrors) -> Self {
        Self::InvalidForm
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
