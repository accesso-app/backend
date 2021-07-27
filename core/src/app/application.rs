use crate::contracts::UnexpectedDatabaseError;
use crate::models;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Application {
    async fn application_get(&self, id: Uuid) -> Result<models::Application, ApplicationGetError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ApplicationGetError {
    #[error("Application not found")]
    ApplicationNotFound,
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

impl From<UnexpectedDatabaseError> for ApplicationGetError {
    fn from(e: UnexpectedDatabaseError) -> Self {
        ApplicationGetError::Unexpected(e.into())
    }
}
