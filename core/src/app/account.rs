use async_trait::async_trait;

use crate::contracts::repo::{UnexpectedDatabaseError, UserEditError};
use crate::models::User;

#[async_trait]
pub trait Account {
    async fn account_edit(
        &self,
        user_id: uuid::Uuid,
        form: AccountEditForm,
    ) -> Result<User, AccountEditError>;
}

#[derive(Debug)]
pub struct AccountEditForm {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountEditError {
    #[error("User not found")]
    UserNotFound,

    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

impl From<UnexpectedDatabaseError> for AccountEditError {
    fn from(_: UnexpectedDatabaseError) -> AccountEditError {
        AccountEditError::Unexpected(eyre::eyre!("Failed to edit an account"))
    }
}

impl From<UserEditError> for AccountEditError {
    fn from(error: UserEditError) -> Self {
        match error {
            UserEditError::UserNotFound => Self::UserNotFound,
            UserEditError::Unexpected(report) => Self::Unexpected(report),
        }
    }
}
