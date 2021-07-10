use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;
use uuid::Uuid;

use super::UnexpectedDatabaseError;
use crate::models::Client;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait ClientRepo {
    async fn client_find_by_id(&self, id: Uuid) -> Result<Option<Client>, UnexpectedDatabaseError>;
}
