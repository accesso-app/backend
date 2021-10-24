use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;

use crate::contracts::UnexpectedDatabaseError;
use crate::models::{Application, User, UserRegistration};

#[derive(Debug, thiserror::Error)]
pub enum UserRegistrationCreateError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    // User already registered in this application
    #[error("User already registered")]
    UserAlreadyRegistered,
    #[error("Client does not exist")]
    ClientDoesNotExist,
    #[error("User does not exist")]
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
        client: &Application,
        user: &User,
    ) -> Result<Option<UserRegistration>, UnexpectedDatabaseError>;

    async fn user_registration_create(
        &self,
        client: &Application,
        user: &User,
    ) -> Result<UserRegistration, UserRegistrationCreateError>;

    async fn user_registration_list_for_client(
        &self,
        application_id: uuid::Uuid,
    ) -> Result<Vec<UserRegistration>, UnexpectedDatabaseError>;

    async fn user_registration_list_for_user(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Vec<UserRegistration>, UnexpectedDatabaseError>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl UserRegistrationsRepo for crate::contracts::MockDb {
    async fn user_registration_get_by_id(
        &self,
        id: sqlx_core::types::Uuid,
    ) -> Result<Option<UserRegistration>, UnexpectedDatabaseError> {
        self.user_registrations
            .user_registration_get_by_id(id)
            .await
    }

    async fn user_registration_find_for_client(
        &self,
        client: &Application,
        user: &User,
    ) -> Result<Option<UserRegistration>, UnexpectedDatabaseError> {
        self.user_registrations
            .user_registration_find_for_client(client, user)
            .await
    }

    async fn user_registration_create(
        &self,
        client: &Application,
        user: &User,
    ) -> Result<UserRegistration, UserRegistrationCreateError> {
        self.user_registrations
            .user_registration_create(client, user)
            .await
    }

    async fn user_registration_list_for_client(
        &self,
        application_id: uuid::Uuid,
    ) -> Result<Vec<UserRegistration>, UnexpectedDatabaseError> {
        self.user_registrations
            .user_registration_list_for_client(application_id)
            .await
    }

    async fn user_registration_list_for_user(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Vec<UserRegistration>, UnexpectedDatabaseError> {
        self.user_registrations
            .user_registration_list_for_user(user_id)
            .await
    }
}
