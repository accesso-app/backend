use crate::contracts::{
    EmailMessage, EmailNotification, RegisterUserError, RequestsRepo, SaveRegisterRequestError,
    SecureGenerator, UnexpectedDatabaseError, UserRegisterForm, UserRepo,
};
use crate::models::RegisterRequest;
use crate::App;
use validator::Validate;

pub trait Registrator {
    fn create_register_request(
        &self,
        form: CreateRegisterRequest,
    ) -> Result<RequestCreated, RegisterRequestError>;

    fn confirm_registration(&self, form: RegisterForm) -> Result<(), RegisterConfirmError>;
}

#[derive(Debug, Clone, Validate, PartialEq, Eq, Hash)]
pub struct CreateRegisterRequest {
    #[validate(email)]
    email: String,
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

#[derive(PartialEq, Eq, Hash)]
pub struct RequestCreated {
    pub expires_at: chrono::NaiveDateTime,
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

const MAX_CODE_INSERT_ATTEMPTS: u8 = 10;

impl<DB, E, G> Registrator for App<DB, E, G>
where
    DB: UserRepo + RequestsRepo,
    G: SecureGenerator,
    E: EmailNotification,
{
    fn create_register_request(
        &self,
        form: CreateRegisterRequest,
    ) -> Result<RequestCreated, RegisterRequestError> {
        form.validate()?;

        let user_exists = self.db.user_has_with_email(form.email.clone())?;

        if user_exists {
            Err(RegisterRequestError::EmailAlreadyRegistered)
        } else {
            let mut generate_count = 0u8;

            let request: RegisterRequest = loop {
                generate_count += 1;

                let code = self.generator.confirmation_code();
                let request = RegisterRequest::new(form.email.clone(), code.clone());
                let result = self.db.register_request_save(request.clone());

                if let Err(SaveRegisterRequestError::CodeAlreadyExists) = result {
                    if generate_count <= MAX_CODE_INSERT_ATTEMPTS {
                        continue;
                    }
                }

                break result;
            }?;

            self.emailer.send(
                form.email,
                EmailMessage::RegisterConfirmation {
                    code: request.code.clone(),
                },
            );

            Ok(RequestCreated {
                expires_at: request.expires_at,
            })
        }
    }

    fn confirm_registration(&self, form: RegisterForm) -> Result<(), RegisterConfirmError> {
        form.validate()?;

        let code = form.confirmation_code.clone();

        if let Some(request) = self.db.register_request_get_by_code(code)? {
            let password_hash = self.generator.password_hash(form.password);

            let created_user = self.db.user_regiser(UserRegisterForm {
                id: uuid::Uuid::new_v4(),
                email: request.email,
                password_hash,
                first_name: form.first_name,
                last_name: form.last_name,
            })?;

            self.emailer.send(
                created_user.email.clone(),
                EmailMessage::RegisterFinished {
                    first_name: created_user.first_name,
                    last_name: created_user.last_name,
                },
            );

            self.db
                .register_requests_delete_all_for_email(created_user.email)?;

            Ok(())
        } else {
            Err(RegisterConfirmError::CodeNotFound)
        }
    }
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

#[cfg(test)]
mod tests {}
