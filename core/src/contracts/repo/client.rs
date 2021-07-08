use super::UnexpectedDatabaseError;
use crate::models::{AccessToken, AuthorizationCode, Client};
use async_trait::async_trait;
use uuid::Uuid;

#[cfg(feature = "testing")]
use mockall::*;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait ClientRepo {
    async fn client_find_by_id(&self, id: Uuid) -> Result<Option<Client>, UnexpectedDatabaseError>;
}

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait AuthCodeRepo {
    async fn auth_code_create(
        &self,
        code: AuthorizationCode,
    ) -> Result<AuthorizationCode, UnexpectedDatabaseError>;

    async fn auth_code_read(
        &self,
        code: String,
    ) -> Result<Option<AuthorizationCode>, UnexpectedDatabaseError>;
}

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait AccessTokenRepo {
    async fn access_token_create(
        &self,
        token: AccessToken,
    ) -> Result<AccessToken, UnexpectedDatabaseError>;
}
