use async_trait::async_trait;
use uuid::Uuid;

use accesso_core::app::application::{
    Application, ApplicationGetError, ApplicationsList, ApplicationsListError,
};
use accesso_core::contracts::Repository;
use accesso_core::models;

use crate::{App, Service};

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

    async fn applications_list(
        &self,
        user_id: Uuid,
    ) -> Result<ApplicationsList, ApplicationsListError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let available_future = db.applications_allowed_to_register();
        let installed_future = db.applications_user_registered_in(user_id);

        let mut available = available_future.await?;
        let installed = installed_future.await?;

        for application in &installed {
            available.retain(|found| found.id != application.id);
        }

        Ok(ApplicationsList {
            available,
            installed,
        })
    }
}
