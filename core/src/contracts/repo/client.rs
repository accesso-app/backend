use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;
use uuid::Uuid;

use super::UnexpectedDatabaseError;
use crate::contracts::MockDb;
use crate::models::Client;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait ClientRepo {
    async fn client_find_by_id(&self, id: Uuid) -> Result<Option<Client>, UnexpectedDatabaseError>;
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
