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

#[derive(Debug, Clone)]
pub struct UserEditForm {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum UserEditError {
    #[error("User not found")]
    UserNotFound,

    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
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
    async fn user_get_by_email(
        &self,
        email: String,
    ) -> Result<Option<User>, UnexpectedDatabaseError>;
    async fn user_edit_by_id(
        &self,
        user_id: uuid::Uuid,
        form: UserEditForm,
    ) -> Result<User, UserEditError>;
    async fn user_list(&self) -> Result<Vec<User>, UnexpectedDatabaseError>;
    async fn user_search(&self, query: String) -> Result<Vec<User>, UnexpectedDatabaseError>;
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

    async fn user_get_by_email(
        &self,
        email: String,
    ) -> Result<Option<User>, UnexpectedDatabaseError> {
        self.users.user_get_by_email(email).await
    }

    async fn user_edit_by_id(
        &self,
        user_id: uuid::Uuid,
        form: UserEditForm,
    ) -> Result<User, UserEditError> {
        self.users.user_edit_by_id(user_id, form).await
    }

    async fn user_list(&self) -> Result<Vec<User>, UnexpectedDatabaseError> {
        self.users.user_list().await
    }

    async fn user_search(&self, query: String) -> Result<Vec<User>, UnexpectedDatabaseError> {
        self.users.user_search(query).await
    }
}
