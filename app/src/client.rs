use crate::{App, Service};
use accesso_core::app::client::{Client, ClientGetError};
use accesso_core::contracts::Repository;
use accesso_core::models;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
impl Client for App {
    async fn client_get(&self, client_id: Uuid) -> Result<models::Client, ClientGetError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let found = db.client_find_by_id(client_id).await?;

        match found {
            Some(client) => Ok(client),
            None => Err(ClientGetError::ClientNotFound),
        }
    }
}
