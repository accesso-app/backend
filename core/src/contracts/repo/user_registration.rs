use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;

use crate::contracts::UnexpectedDatabaseError;
use crate::models::{Client, User, UserRegistration};

#[derive(Debug, PartialEq, Eq)]
pub enum UserRegistrationCreateError {
    Unexpected,
    // User already registered in this client
    UserAlreadyRegistered,
    ClientDoesNotExist,
    UserDoesNotExist,
}

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait UserRegistrationsRepo {
    async fn user_registration_get_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<UserRegistration>, UnexpectedDatabaseError>;

    async fn user_registration_find_for_client(
        &self,
        client: &Client,
        user: &User,
    ) -> Result<Option<UserRegistration>, UnexpectedDatabaseError>;

    async fn user_registration_create(
        &self,
        client: &Client,
        user: &User,
    ) -> Result<UserRegistration, UserRegistrationCreateError>;
}
