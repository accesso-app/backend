use accesso_app::Service;
use accesso_core::contracts::{Repository, SecureGenerator};
use actix_web::web::Data;
use actix_web::HttpResponse;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema as GraphQlSchema};
use async_graphql_actix_web::{Request, Response};
use std::sync::Arc;

#[derive(GraphQLObject, Default, Clone)]
#[graphql(description = "The application user want to register")]
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

#[derive(GraphQLObject, Default, Clone)]
#[graphql(description = "The application user want to register")]
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

#[derive(GraphQLInputObject)]
struct ApplicationCreate {
    title: String,
    redirect_uri: Vec<String>,
    is_dev: Option<bool>,
    allowed_registrations: Option<bool>,
}

pub struct Context {
    app: Arc<accesso_app::App>,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
        "0.1"
    }

    async fn application(context: &Context, id: uuid::Uuid) -> FieldResult<Option<Application>> {
        let db = context.app.get::<Service<dyn Repository>>()?;
        let found = db.application_find_by_id(id).await?;

        Ok(found.map(|app| app.into()))
    }

    async fn applications(context: &Context) -> FieldResult<Vec<Application>> {
        let db = context.app.get::<Service<dyn Repository>>()?;
        let list = db.application_list().await?;
        Ok(list.into_iter().map(|client| client.into()).collect())
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn applicationCreate(
        context: &Context,
        form: ApplicationCreate,
    ) -> FieldResult<ApplicationSecret> {
        let db = context.app.get::<Service<dyn Repository>>()?;
        let generator = context.app.get::<Service<dyn SecureGenerator>>()?;
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

    async fn applicationRegenerateSecret(
        context: &Context,
        application_id: uuid::Uuid,
    ) -> FieldResult<Option<ApplicationSecret>> {
        let db = context.app.get::<Service<dyn Repository>>()?;
        let generator = context.app.get::<Service<dyn SecureGenerator>>()?;
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

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}

pub async fn playground_route() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql"))))
}

pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: Data<Schema>,
    app: Data<accesso_app::App>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    juniper_actix::graphql_handler(
        &schema,
        &Context {
            app: app.into_inner(),
        },
        req,
        payload,
    )
    .await
}
