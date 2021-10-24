use async_graphql::{
    ComplexObject, Context, EmptySubscription, InputObject, Object, Schema, SchemaBuilder,
    SimpleObject,
};

use accesso_app::Service;
use accesso_core::contracts::{Repository, SecureGenerator};

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

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct UserRegistration {
    pub id: uuid::Uuid,
    /// Field renamed from `client_id`
    pub application_id: uuid::Uuid,
    // User registration does not expires!
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user_id: uuid::Uuid,
}

impl From<accesso_core::models::UserRegistration> for UserRegistration {
    fn from(ur: accesso_core::models::UserRegistration) -> Self {
        Self {
            id: ur.id,
            application_id: ur.client_id,
            created_at: ur.created_at,
            user_id: ur.user_id,
        }
    }
}

#[ComplexObject]
impl UserRegistration {
    async fn user(&self, context: &Context<'_>) -> async_graphql::Result<Option<User>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let user = db.user_get_by_id(self.user_id).await?;
        Ok(user.map(Into::into))
    }

    async fn application(
        &self,
        context: &Context<'_>,
    ) -> async_graphql::Result<Option<Application>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let application = db.application_find_by_id(self.application_id).await?;
        Ok(application.map(Into::into))
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

#[derive(InputObject)]
struct ApplicationCreate {
    title: String,
    redirect_uri: Vec<String>,
    is_dev: Option<bool>,
    allowed_registrations: Option<bool>,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct User {
    id: uuid::Uuid,
    email: String,
    canonical_email: String,
    first_name: String,
    last_name: String,
}

impl From<accesso_core::models::User> for User {
    fn from(user: accesso_core::models::User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            canonical_email: user.canonical_email,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }
}

#[ComplexObject]
impl User {
    async fn registrations(
        &self,
        context: &Context<'_>,
    ) -> async_graphql::Result<Vec<UserRegistration>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let apps = db.user_registration_list_for_user(self.id).await?;
        Ok(apps.into_iter().map(Into::into).collect())
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn version(&self) -> &'static str {
        "0.1"
    }

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
        Ok(list.into_iter().map(|client| client.into()).collect())
    }

    async fn users(&self, context: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let list = db.user_list().await?;
        Ok(list.into_iter().map(|client| client.into()).collect())
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
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

pub type AdminSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription).limit_depth(8)
}
