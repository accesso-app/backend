use crate::{App, Service};
use accesso_core::app::application::{Application, ApplicationGetError};
use accesso_core::contracts::Repository;
use accesso_core::models;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
impl Application for App {
    async fn application_get(
        &self,
        application_id: Uuid,
    ) -> Result<models::Application, ApplicationGetError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let found = db.application_find_by_id(application_id).await?;

        match found {
            Some(client) => Ok(client),
            None => Err(ApplicationGetError::ApplicationNotFound),
        }
    }
}
