use crate::models::{CreatedUser, RegisterRequest};

#[derive(Debug)]
pub struct UnexpectedDatabaseError;

pub trait UserRepo {
    fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError>;
    fn register_user(&self, form: UserRegisterForm) -> Result<CreatedUser, RegisterUserError>;
}

pub trait RequestsRepo {
    fn register_request_save(
        &self,
        request: RegisterRequest,
    ) -> Result<RegisterRequest, SaveRegisterRequestError>;

    /// Find actual register request by its code
    fn register_request_get_by_code(
        &self,
        code: String,
    ) -> Result<Option<RegisterRequest>, UnexpectedDatabaseError>;

    fn register_requests_delete_all_for_email(
        &self,
        email: String,
    ) -> Result<usize, UnexpectedDatabaseError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum SaveRegisterRequestError {
    Unexpected,
    CodeAlreadyExists,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegisterUserError {
    Unexpected,
    EmailAlreadyExists,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserRegisterForm {
    pub id: uuid::Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}
