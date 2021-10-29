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

    async fn access_tokens_list(&self) -> Result<Vec<AccessToken>, UnexpectedDatabaseError>;

    async fn access_tokens_list_for_registration(
        &self,
        registration_id: uuid::Uuid,
    ) -> Result<Vec<AccessToken>, UnexpectedDatabaseError>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl AccessTokenRepo for crate::contracts::MockDb {
    async fn access_token_create(
        &self,
        token: AccessToken,
    ) -> Result<AccessToken, UnexpectedDatabaseError> {
        self.access_token.access_token_create(token).await
    }

    async fn access_tokens_list(&self) -> Result<Vec<AccessToken>, UnexpectedDatabaseError> {
        self.access_token.access_tokens_list().await
    }

    async fn access_tokens_list_for_registration(
        &self,
        registration_id: uuid::Uuid,
    ) -> Result<Vec<AccessToken>, UnexpectedDatabaseError> {
        self.access_token
            .access_tokens_list_for_registration(registration_id)
            .await
    }
}
