use crate::models::RegisterRequest;

#[derive(Debug)]
pub struct UnexpectedDatabaseError;

pub trait UserRepo {
    fn has_user_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum SaveRegisterRequestError {
    UnexpectedError,
    CodeAlreadyExists,
}

pub trait RequestsRepo {
    fn save_register_request(
        &self,
        request: RegisterRequest,
    ) -> Result<RegisterRequest, SaveRegisterRequestError>;
}
