#[cfg(feature = "testing")]
use crate::models::{AccessToken, AuthorizationCode, Client};
use crate::models::{RegisterRequest, SessionToken, User};

pub use client::*;

mod client;

#[cfg(feature = "testing")]
use mockall::*;

#[derive(Debug)]
pub struct UnexpectedDatabaseError;

#[cfg_attr(feature = "testing", automock)]
pub trait UserRepo {
    fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError>;
    fn user_register(&self, form: UserRegisterForm) -> Result<User, RegisterUserError>;
    fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<User>, UnexpectedDatabaseError>;
}

#[cfg_attr(feature = "testing", automock)]
pub trait SessionRepo {
    fn get_user_by_session_token(&self, token: String) -> Result<User, GetUserBySessionError>;
    fn get_user_by_access_token(&self, token: String) -> Result<User, GetUserBySessionError>;
    fn session_create(&self, session: SessionToken) -> Result<SessionToken, SessionCreateError>;
    fn session_delete_token(&self, session_token: &str) -> Result<(), UnexpectedDatabaseError>;
    fn session_delete_by_user_id(&self, user_id: uuid::Uuid)
        -> Result<(), UnexpectedDatabaseError>;
}

#[cfg_attr(feature = "testing", automock)]
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

#[cfg(feature = "testing")]
pub struct MockDb {
    pub users: MockUserRepo,
    pub requests: MockRequestsRepo,
    pub session: MockSessionRepo,
    pub auth_code: MockAuthCodeRepo,
    pub client: MockClientRepo,
    pub access_token: MockAccessTokenRepo,
}

#[cfg(feature = "testing")]
impl Default for MockDb {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "testing")]
impl MockDb {
    pub fn new() -> Self {
        Self {
            users: MockUserRepo::new(),
            requests: MockRequestsRepo::new(),
            session: MockSessionRepo::new(),
            access_token: MockAccessTokenRepo::new(),
            auth_code: MockAuthCodeRepo::new(),
            client: MockClientRepo::new(),
        }
    }
}

#[cfg(feature = "testing")]
impl UserRepo for MockDb {
    fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError> {
        self.users.user_has_with_email(email)
    }
    fn user_register(&self, form: UserRegisterForm) -> Result<User, RegisterUserError> {
        self.users.user_register(form)
    }
    fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<User>, UnexpectedDatabaseError> {
        self.users.user_find_by_credentials(creds)
    }
}

#[cfg(feature = "testing")]
impl RequestsRepo for MockDb {
    fn register_request_save(
        &self,
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
        &self,
        email: String,
    ) -> Result<usize, UnexpectedDatabaseError> {
        self.requests.register_requests_delete_all_for_email(email)
    }
}

#[cfg(feature = "testing")]
impl SessionRepo for MockDb {
    fn get_user_by_session_token(&self, token: String) -> Result<User, GetUserBySessionError> {
        self.session.get_user_by_session_token(token)
    }
    fn get_user_by_access_token(&self, token: String) -> Result<User, GetUserBySessionError> {
        self.session.get_user_by_access_token(token)
    }
    fn session_create(&self, session: SessionToken) -> Result<SessionToken, SessionCreateError> {
        self.session.session_create(session)
    }

    fn session_delete_token(&self, session_token: &str) -> Result<(), UnexpectedDatabaseError> {
        self.session.session_delete_token(session_token)
    }
    fn session_delete_by_user_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<(), UnexpectedDatabaseError> {
        self.session.session_delete_by_user_id(user_id)
    }
}

#[cfg(feature = "testing")]
impl ClientRepo for MockDb {
    fn client_find_by_id(&self, id: uuid::Uuid) -> Result<Option<Client>, UnexpectedDatabaseError> {
        self.client.client_find_by_id(id)
    }
}

#[cfg(feature = "testing")]
impl AuthCodeRepo for MockDb {
    fn auth_code_create(
        &self,
        code: AuthorizationCode,
    ) -> Result<AuthorizationCode, UnexpectedDatabaseError> {
        self.auth_code.auth_code_create(code)
    }

    fn auth_code_read(
        &self,
        code: String,
    ) -> Result<Option<AuthorizationCode>, UnexpectedDatabaseError> {
        self.auth_code.auth_code_read(code)
    }
}

#[cfg(feature = "testing")]
impl AccessTokenRepo for MockDb {
    fn access_token_create(
        &self,
        token: AccessToken,
    ) -> Result<AccessToken, UnexpectedDatabaseError> {
        self.access_token.access_token_create(token)
    }
}
