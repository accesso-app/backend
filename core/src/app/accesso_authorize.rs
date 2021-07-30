use crate::contracts::UnexpectedDatabaseError;
use crate::models;

#[async_trait]
pub trait AccessoAuthorize {
    async fn authorize(
        &self,
        user: AdminUserInfo,
    ) -> Result<(models::AdminUser, models::AdminSessionToken), UpdateAdminUserFailure>;
}

#[derive(Debug, Clone)]
pub struct AdminUserInfo {
    pub accesso_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateAdminUserFailure {
    #[error("Unexpected update user failure: {0}")]
    Unexpected(#[from] eyre::Report),
}

impl From<UnexpectedDatabaseError> for UpdateAdminUserFailure {
    fn from(e: UnexpectedDatabaseError) -> Self {
        Self::Unexpected(e.into())
    }
}
