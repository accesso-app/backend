use crate::models::{RegisterRequest, SessionToken, User};

#[cfg(test)]
use mockall::*;

#[derive(Debug)]
pub struct UnexpectedDatabaseError;

#[cfg_attr(test, automock)]
pub trait UserRepo {
    fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError>;
    fn user_register(&mut self, form: UserRegisterForm) -> Result<User, RegisterUserError>;
    fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<User>, UnexpectedDatabaseError>;
}

#[cfg_attr(test, automock)]
pub trait SessionRepo {
    fn get_user_by_session_token(&self, token: String) -> Result<User, GetUserBySessionError>;
    fn session_create(&mut self, session: SessionToken)
        -> Result<SessionToken, SessionCreateError>;
}

#[cfg_attr(test, automock)]
pub trait RequestsRepo {
    fn register_request_save(
        &mut self,
        request: RegisterRequest,
    ) -> Result<RegisterRequest, SaveRegisterRequestError>;

    /// Find actual register request by its code
    fn register_request_get_by_code(
        &self,
        code: String,
    ) -> Result<Option<RegisterRequest>, UnexpectedDatabaseError>;

    fn register_requests_delete_all_for_email(
        &mut self,
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

#[cfg(test)]
pub struct MockDb {
    pub users: MockUserRepo,
    pub requests: MockRequestsRepo,
}

#[cfg(test)]
impl MockDb {
    pub fn new() -> Self {
        Self {
            users: MockUserRepo::new(),
            requests: MockRequestsRepo::new(),
        }
    }
}

#[cfg(test)]
impl UserRepo for MockDb {
    fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError> {
        self.users.user_has_with_email(email)
    }
    fn user_register(&mut self, form: UserRegisterForm) -> Result<User, RegisterUserError> {
        self.users.user_register(form)
    }
    fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<User>, UnexpectedDatabaseError> {
        self.users.user_find_by_credentials(creds)
    }
}

#[cfg(test)]
impl RequestsRepo for MockDb {
    fn register_request_save(
        &mut self,
        request: RegisterRequest,
    ) -> Result<RegisterRequest, SaveRegisterRequestError> {
        self.requests.register_request_save(request)
    }

    /// Find actual register request by its code
    fn register_request_get_by_code(
        &self,
        code: String,
    ) -> Result<Option<RegisterRequest>, UnexpectedDatabaseError> {
        self.requests.register_request_get_by_code(code)
    }

    fn register_requests_delete_all_for_email(
        &mut self,
        email: String,
    ) -> Result<usize, UnexpectedDatabaseError> {
        self.requests.register_requests_delete_all_for_email(email)
    }
}
