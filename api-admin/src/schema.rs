use async_graphql::{
    Context, EmptySubscription, InputObject, Object, Schema, SchemaBuilder, SimpleObject,
};

use accesso_app::Service;
use accesso_core::contracts::Repository;

#[derive(SimpleObject, Default)]
pub struct Application {
    id: uuid::Uuid,
    is_dev: bool,
    redirect_uri: Vec<String>,
    title: String,
    allowed_registrations: bool,
}

impl From<accesso_core::models::Application> for Application {
    fn from(app: accesso_core::models::Application) -> Self {
        Application {
            id: app.id,
            is_dev: app.is_dev,
            redirect_uri: app.redirect_uri,
            title: app.title,
            allowed_registrations: app.allowed_registrations,
        }
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
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn demo(&self) -> &'static str {
        "hello"
    }
}

pub type AdminSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription)
}
