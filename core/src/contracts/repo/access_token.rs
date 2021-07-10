use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;

use crate::contracts::UnexpectedDatabaseError;
use crate::models::AccessToken;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait AccessTokenRepo {
    async fn access_token_create(
        &self,
        token: AccessToken,
    ) -> Result<AccessToken, UnexpectedDatabaseError>;
}
