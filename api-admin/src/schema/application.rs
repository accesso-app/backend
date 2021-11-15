use async_graphql::{ComplexObject, Context, InputObject, Object, SimpleObject};
use uuid::Uuid;

use accesso_app::Service;
use accesso_core::contracts::{Repository, SecureGenerator};

use super::user_registration::UserRegistration;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Application {
    id: uuid::Uuid,
    is_dev: bool,
    redirect_uri: Vec<String>,
    title: String,
    allowed_registrations: bool,
    #[graphql(skip)]
    secret_key: String,
}

impl From<accesso_core::models::Application> for Application {
    fn from(app: accesso_core::models::Application) -> Self {
        Application {
            id: app.id,
            is_dev: app.is_dev,
            redirect_uri: app.redirect_uri,
            title: app.title,
            allowed_registrations: app.allowed_registrations,
            secret_key: app.secret_key,
        }
    }
}

impl Into<accesso_core::models::Application> for Application {
    fn into(self) -> accesso_core::models::Application {
        accesso_core::models::Application {
            id: self.id,
            is_dev: self.is_dev,
            redirect_uri: self.redirect_uri,
            title: self.title,
            secret_key: self.secret_key,
            allowed_registrations: self.allowed_registrations,
        }
    }
}

#[ComplexObject]
impl Application {
    async fn registrations(
        &self,
        context: &Context<'_>,
    ) -> async_graphql::Result<Vec<UserRegistration>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db
            .user_registration_list_for_client(self.id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

#[derive(SimpleObject, Default, Clone)]
pub struct ApplicationSecret {
    id: uuid::Uuid,
    is_dev: bool,
    // TODO: use `url::Url` here
    redirect_uri: Vec<String>,
    title: String,
    allowed_registrations: bool,
    /// Allowed to read only after application is created
    secret_key: String,
}

impl From<accesso_core::models::Application> for ApplicationSecret {
    fn from(app: accesso_core::models::Application) -> Self {
        ApplicationSecret {
            id: app.id,
            is_dev: app.is_dev,
            redirect_uri: app.redirect_uri,
            title: app.title,
            allowed_registrations: app.allowed_registrations,
            secret_key: app.secret_key,
        }
    }
}

#[derive(Default)]
pub struct QueryApplication;

#[Object]
impl QueryApplication {
    async fn application(
        &self,
        context: &Context<'_>,
        id: uuid::Uuid,
    ) -> async_graphql::Result<Option<Application>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let found = db.application_find_by_id(id).await?;

        Ok(found.map(|app| app.into()))
    }

    async fn applications(&self, context: &Context<'_>) -> async_graphql::Result<Vec<Application>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let list = db.application_list().await?;
        Ok(list.into_iter().map(Into::into).collect())
    }
}

#[derive(Default)]
pub struct MutationApplication;

#[derive(InputObject)]
struct ApplicationCreate {
    title: String,
    redirect_uri: Vec<String>,
    is_dev: Option<bool>,
    allowed_registrations: Option<bool>,
}

#[derive(InputObject)]
struct ApplicationEdit {
    id: Uuid,
    title: Option<String>,
    redirect_uri: Option<Vec<String>>,
    is_dev: Option<bool>,
    allowed_registrations: Option<bool>,
}

#[Object]
impl MutationApplication {
    async fn application_create(
        &self,
        context: &Context<'_>,
        form: ApplicationCreate,
    ) -> async_graphql::Result<ApplicationSecret> {
        let db = context.data::<Service<dyn Repository>>()?;
        let generator = context.data::<Service<dyn SecureGenerator>>()?;
        let app = db
            .application_create(accesso_core::contracts::ApplicationForm {
                title: form.title,
                redirect_uri: form.redirect_uri,
                is_dev: form.is_dev.unwrap_or_default(),
                allowed_registrations: form.allowed_registrations.unwrap_or_default(),
                secret_key: generator.generate_token_long(),
            })
            .await?;

        Ok(app.into())
    }

    async fn application_edit(
        &self,
        context: &Context<'_>,
        form: ApplicationEdit,
    ) -> async_graphql::Result<Option<Application>> {
        let db = context.data::<Service<dyn Repository>>()?;
        if let Some(app) = db.application_find_by_id(form.id).await? {
            let allowed_registrations = form.allowed_registrations.unwrap_or(app.allowed_registrations);
            let is_dev = form.is_dev.unwrap_or(app.is_dev);
            let redirect_uri = form.redirect_uri.unwrap_or(app.redirect_uri);
            let title = form.title.unwrap_or(app.title);
            let app = db
                .application_edit(form.id, accesso_core::contracts::ApplicationForm {
                    title,
                    redirect_uri,
                    is_dev,
                    allowed_registrations,
                    secret_key: app.secret_key,
                })
                .await?;
            if let Some(app) = app {
                Ok(Some(app.into()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }

    }

    async fn application_regenerate_secret(
        &self,
        context: &Context<'_>,
        application_id: uuid::Uuid,
    ) -> async_graphql::Result<Option<ApplicationSecret>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let generator = context.data::<Service<dyn SecureGenerator>>()?;
        let app = db.application_find_by_id(application_id).await?;
        if let Some(app) = app {
            let updated = db
                .application_edit(
                    application_id,
                    accesso_core::contracts::ApplicationForm {
                        secret_key: generator.generate_token_long(),
                        ..app.into()
                    },
                )
                .await?;
            Ok(updated.map(|app| app.into()))
        } else {
            Ok(None)
        }
    }
}
