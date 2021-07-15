use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;

use crate::contracts::UnexpectedDatabaseError;
use crate::models::AuthorizationCode;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait AuthCodeRepo {
    async fn auth_code_create(
        &self,
        code: AuthorizationCode,
    ) -> Result<AuthorizationCode, UnexpectedDatabaseError>;

    async fn auth_code_read(
        &self,
        code: String,
    ) -> Result<Option<AuthorizationCode>, UnexpectedDatabaseError>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl AuthCodeRepo for crate::contracts::MockDb {
    async fn auth_code_create(
        &self,
        code: AuthorizationCode,
    ) -> Result<AuthorizationCode, UnexpectedDatabaseError> {
        self.auth_code.auth_code_create(code).await
    }

    async fn auth_code_read(
        &self,
        code: String,
    ) -> Result<Option<AuthorizationCode>, UnexpectedDatabaseError> {
        self.auth_code.auth_code_read(code).await
    }
}
