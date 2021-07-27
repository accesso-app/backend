use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;
use uuid::Uuid;

use super::UnexpectedDatabaseError;
use crate::models::Application;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait ApplicationRepo {
    async fn application_find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Application>, UnexpectedDatabaseError>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl ApplicationRepo for crate::contracts::MockDb {
    async fn application_find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<Application>, UnexpectedDatabaseError> {
        self.application.application_find_by_id(id).await
    }
}
