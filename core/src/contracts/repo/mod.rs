#[cfg(feature = "testing")]
use mockall::*;

pub use access_token::*;
pub use auth_code::*;
pub use client::*;
pub use requests::*;
pub use session::*;
pub use user::*;
pub use user_registration::*;

mod access_token;
mod auth_code;
mod client;
mod requests;
mod session;
mod user;
mod user_registration;

#[derive(Debug)]
pub struct UnexpectedDatabaseError;

#[cfg(feature = "testing")]
use crate::models::{AccessToken, AuthorizationCode, Client};

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
