use super::RepoResult;
use crate::models;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg(feature = "testing")]
use mockall::*;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait AdminUserRepo {
    async fn user_find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::AdminUser>>;
    async fn user_find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::AdminUser>>;
    async fn user_update(&self, user: models::AdminUser) -> RepoResult<models::AdminUser>;
    async fn user_create(&self, user: models::AdminUserCreate) -> Result<models::AdminUser, AdminUserCreateError>;
}

#[derive(Debug, thiserror::Error)]
pub enum AdminUserCreateError {
    #[error("AdminUser already exists")]
    AdminUserAlreadyExists,
    #[error(transparent)]
    UnexpectedFailure(#[from] eyre::Report),
}

#[cfg(feature = "testing")]
#[async_trait]
impl AdminUserRepo for crate::contracts::MockDb {
    async fn user_find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::AdminUser>> {
        self.users.user_find_by_id(user_id).await
    }

    async fn user_find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::AdminUser>> {
        self.users.user_find_by_accesso(accesso_id).await
    }

    async fn user_update(&self, user: models::AdminUser) -> RepoResult<models::AdminUser> {
        self.users.user_update(user).await
    }

    async fn user_create(&self, user: models::AdminUserCreate) -> Result<models::AdminUser, AdminUserCreateError> {
        self.users.user_create(user).await
    }
}
