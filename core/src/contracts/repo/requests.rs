use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;

use crate::contracts::UnexpectedDatabaseError;
use crate::models::RegisterRequest;

#[derive(Debug, thiserror::Error)]
pub enum SaveRegisterRequestError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("Code already exists")]
    CodeAlreadyExists,
}

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait RequestsRepo {
    async fn register_request_save(
        &self,
        request: RegisterRequest,
    ) -> Result<RegisterRequest, SaveRegisterRequestError>;

    /// Find actual register request by its code
    async fn register_request_get_by_code(
        &self,
        code: String,
    ) -> Result<Option<RegisterRequest>, UnexpectedDatabaseError>;

    async fn register_requests_delete_all_for_email(
        &self,
        email: String,
    ) -> Result<u64, UnexpectedDatabaseError>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl RequestsRepo for crate::contracts::MockDb {
    async fn register_request_save(
        &self,
        request: RegisterRequest,
    ) -> Result<RegisterRequest, SaveRegisterRequestError> {
        self.requests.register_request_save(request).await
    }

    /// Find actual register request by its code
    async fn register_request_get_by_code(
        &self,
        code: String,
    ) -> Result<Option<RegisterRequest>, UnexpectedDatabaseError> {
        self.requests.register_request_get_by_code(code).await
    }

    async fn register_requests_delete_all_for_email(
        &self,
        email: String,
    ) -> Result<u64, UnexpectedDatabaseError> {
        self.requests
            .register_requests_delete_all_for_email(email)
            .await
    }
}
