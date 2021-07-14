use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;

use crate::contracts::UnexpectedDatabaseError;
use crate::models::User;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserRegisterForm {
    pub id: uuid::Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserCredentials {
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RegisterUserError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("Email already exists")]
    EmailAlreadyExists,
}

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait UserRepo {
    async fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError>;
    async fn user_register(&self, form: UserRegisterForm) -> Result<User, RegisterUserError>;
    async fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<User>, UnexpectedDatabaseError>;
    async fn user_get_by_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Option<User>, UnexpectedDatabaseError>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl UserRepo for crate::contracts::MockDb {
    async fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError> {
        self.users.user_has_with_email(email).await
    }
    async fn user_register(&self, form: UserRegisterForm) -> Result<User, RegisterUserError> {
        self.users.user_register(form).await
    }
    async fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<User>, UnexpectedDatabaseError> {
        self.users.user_find_by_credentials(creds).await
    }

    async fn user_get_by_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Option<User>, UnexpectedDatabaseError> {
        self.users.user_get_by_id(user_id).await
    }
}
