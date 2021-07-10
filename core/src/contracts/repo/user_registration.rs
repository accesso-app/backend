use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;

use crate::contracts::UnexpectedDatabaseError;
use crate::models::UserRegistration;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait UserRegistrationsRepo {
    async fn user_registration_get_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<UserRegistration>, UnexpectedDatabaseError>;
}
