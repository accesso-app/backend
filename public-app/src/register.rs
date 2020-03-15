use crate::contracts::{
    RegisterEmailer, RequestsRepo, SaveRegisterRequestError, SecureGenerator,
    UnexpectedDatabaseError, UserRepo,
};
use crate::models::RegisterRequest;
use crate::App;

pub struct RequestCreated {
    pub expires_at: chrono::NaiveDateTime,
}
pub trait Registrator {
    fn create_register_request(
        &self,
        email: String,
    ) -> Result<RequestCreated, RegisterRequestError>;
}

#[derive(Debug)]
pub enum RegisterRequestError {
    UnexpectedError,
    EmailAlreadyRegistered,
}

const MAX_CODE_INSERT_ATTEMPTS: u8 = 10;

impl<DB, E, G> Registrator for App<DB, E, G>
where
    DB: UserRepo + RequestsRepo,
    G: SecureGenerator,
    E: RegisterEmailer,
{
    fn create_register_request(
        &self,
        email: String,
    ) -> Result<RequestCreated, RegisterRequestError> {
        let user_exists = self.db.has_user_with_email(email.clone())?;

        if user_exists {
            Err(RegisterRequestError::EmailAlreadyRegistered)
        } else {
            let mut generate_count = 0u8;

            let request: RegisterRequest = loop {
                generate_count += 1;

                let code = self.generator.confirmation_code();
                let request = RegisterRequest::new(email.clone(), code.clone());
                let result = self.db.save_register_request(request.clone());

                if let Err(SaveRegisterRequestError::CodeAlreadyExists) = result {
                    if generate_count > MAX_CODE_INSERT_ATTEMPTS {
                        break result;
                    }
                    continue;
                }

                break result;
            }?;

            self.emailer.confirmation_code(email, request.code);

            Ok(RequestCreated {
                expires_at: request.expires_at,
            })
        }
    }
}

impl From<UnexpectedDatabaseError> for RegisterRequestError {
    fn from(_: UnexpectedDatabaseError) -> Self {
        RegisterRequestError::UnexpectedError
    }
}

impl From<SaveRegisterRequestError> for RegisterRequestError {
    fn from(_: SaveRegisterRequestError) -> Self {
        // Now all errors from request errors converted to Unexpected
        // Because CodeAlreadyExists is handled at create_register_request impl
        Self::UnexpectedError
    }
}

#[cfg(test)]
mod tests {}
