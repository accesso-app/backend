#[cfg(feature = "testing")]
use crate::models::{AccessToken, AuthorizationCode, Client};
use crate::models::{RegisterRequest, SessionToken, User};
use async_trait::async_trait;

pub use client::*;

mod client;

#[cfg(feature = "testing")]
use mockall::*;

#[derive(Debug)]
pub struct UnexpectedDatabaseError;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait UserRepo {
    async fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError>;
    async fn user_register(&self, form: UserRegisterForm) -> Result<User, RegisterUserError>;
    async fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<User>, UnexpectedDatabaseError>;
}

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait SessionRepo {
    async fn get_user_by_session_token(&self, token: String)
        -> Result<User, GetUserBySessionError>;
    async fn get_user_by_access_token(&self, token: String) -> Result<User, GetUserBySessionError>;
    async fn session_create(
        &self,
        session: SessionToken,
    ) -> Result<SessionToken, SessionCreateError>;
    async fn session_delete_token(
        &self,
        session_token: &str,
    ) -> Result<(), UnexpectedDatabaseError>;
    async fn session_delete_by_user_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<(), UnexpectedDatabaseError>;
}

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait RequestsRepo {
    async fn register_request_save(
        &self,
        request: RegisterRequest,
    ) -> Result<RegisterRequest, SaveRegisterRequestError>;

    /// Find actual register request by its code
    async fn register_request_get_by_code(
        &self,
        code: String,
    ) -> Result<Option<RegisterRequest>, UnexpectedDatabaseError>;

    async fn register_requests_delete_all_for_email(
        &self,
        email: String,
    ) -> Result<u64, UnexpectedDatabaseError>;
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
#[async_trait]
impl UserRepo for MockDb {
    async fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError> {
        self.users.user_has_with_email(email).await
    }
    async fn user_register(&self, form: UserRegisterForm) -> Result<User, RegisterUserError> {
        self.users.user_register(form).await
    }
    async fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<User>, UnexpectedDatabaseError> {
        self.users.user_find_by_credentials(creds).await
    }
}

#[cfg(feature = "testing")]
#[async_trait]
impl RequestsRepo for MockDb {
    async fn register_request_save(
        &self,
        request: RegisterRequest,
    ) -> Result<RegisterRequest, SaveRegisterRequestError> {
        self.requests.register_request_save(request).await
    }

    /// Find actual register request by its code
    async fn register_request_get_by_code(
        &self,
        code: String,
    ) -> Result<Option<RegisterRequest>, UnexpectedDatabaseError> {
        self.requests.register_request_get_by_code(code).await
    }

    async fn register_requests_delete_all_for_email(
        &self,
        email: String,
    ) -> Result<u64, UnexpectedDatabaseError> {
        self.requests
            .register_requests_delete_all_for_email(email)
            .await
    }
}

#[cfg(feature = "testing")]
#[async_trait]
impl SessionRepo for MockDb {
    async fn get_user_by_session_token(
        &self,
        token: String,
    ) -> Result<User, GetUserBySessionError> {
        self.session.get_user_by_session_token(token).await
    }
    async fn get_user_by_access_token(&self, token: String) -> Result<User, GetUserBySessionError> {
        self.session.get_user_by_access_token(token).await
    }
    async fn session_create(
        &self,
        session: SessionToken,
    ) -> Result<SessionToken, SessionCreateError> {
        self.session.session_create(session).await
    }

    async fn session_delete_token(
        &self,
        session_token: &str,
    ) -> Result<(), UnexpectedDatabaseError> {
        self.session.session_delete_token(session_token).await
    }
    async fn session_delete_by_user_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<(), UnexpectedDatabaseError> {
        self.session.session_delete_by_user_id(user_id).await
    }
}

#[cfg(feature = "testing")]
#[async_trait]
impl ClientRepo for MockDb {
    async fn client_find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<Client>, UnexpectedDatabaseError> {
        self.client.client_find_by_id(id).await
    }
}

#[cfg(feature = "testing")]
#[async_trait]
impl AuthCodeRepo for MockDb {
    async fn auth_code_create(
        &self,
        code: AuthorizationCode,
    ) -> Result<AuthorizationCode, UnexpectedDatabaseError> {
        self.auth_code.auth_code_create(code).await
    }

    async fn auth_code_read(
        &self,
        code: String,
    ) -> Result<Option<AuthorizationCode>, UnexpectedDatabaseError> {
        self.auth_code.auth_code_read(code).await
    }
}

#[cfg(feature = "testing")]
#[async_trait]
impl AccessTokenRepo for MockDb {
    async fn access_token_create(
        &self,
        token: AccessToken,
    ) -> Result<AccessToken, UnexpectedDatabaseError> {
        self.access_token.access_token_create(token).await
    }
}
