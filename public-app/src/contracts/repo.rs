use crate::models::{RegisterRequest, SessionToken, User};

#[derive(Debug)]
pub struct UnexpectedDatabaseError;

pub trait UserRepo {
    fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError>;
    fn user_register(&self, form: UserRegisterForm) -> Result<User, RegisterUserError>;
    fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<User>, UnexpectedDatabaseError>;
}

pub trait SessionRepo {
    fn get_user_by_session_token(&self, token: String) -> Result<User, GetUserBySessionError>;
    fn session_create(&self, session: SessionToken) -> Result<SessionToken, SessionCreateError>;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserCredentials {
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetUserBySessionError {
    Unexpected,
    NotFound,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SessionCreateError {
    Unexpected,
    TokenAlreadyExists,
    UserNotFound,
}
