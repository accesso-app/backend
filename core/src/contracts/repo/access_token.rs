use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;

use crate::contracts::{MockDb, UnexpectedDatabaseError};
use crate::models::AccessToken;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait AccessTokenRepo {
    async fn access_token_create(
        &self,
        token: AccessToken,
    ) -> Result<AccessToken, UnexpectedDatabaseError>;
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
