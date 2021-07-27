use crate::contracts::UnexpectedDatabaseError;
use crate::models;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Client {
    async fn client_get(&self, id: Uuid) -> Result<models::Client, ClientGetError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ClientGetError {
    #[error("Client not found")]
    ClientNotFound,
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

impl From<UnexpectedDatabaseError> for ClientGetError {
    fn from(e: UnexpectedDatabaseError) -> Self {
        ClientGetError::Unexpected(e.into())
    }
}
