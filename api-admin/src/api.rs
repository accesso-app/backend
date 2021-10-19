use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject,
    GraphQLObject, ScalarValue,
};

#[derive(GraphQLObject, Default, Clone)]
#[graphql(description = "The application user want to register")]
pub struct Application {
    id: uuid::Uuid,
    is_dev: bool,
    redirect_uri: Vec<String>,
    secret_key: String,
    title: String,
    allowed_registrations: bool,
}

impl From<accesso_core::models::Application> for Application {
    fn from(app: accesso_core::models::Application) -> Self {
        Application {
            id: app.id,
            is_dev: app.is_dev,
            redirect_uri: app.redirect_uri,
            secret_key: app.secret_key,
            title: app.title,
            allowed_registrations: app.allowed_registrations,
        }
    }
}

use accesso_core::contracts::{Repository, SecureGenerator};
use std::sync::Arc;

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

    async fn application(context: &Context, id: uuid::Uuid) -> FieldResult<Application> {
        use accesso_core::app::application::Application;
        let found = context.app.application_get(id).await?;

        Ok(found.into())
    }
}

pub struct Mutation;

use std::fmt::Display;

#[graphql_object(context = Context)]
impl Mutation {
    async fn createApplication(context: &Context) -> FieldResult<Application> {
        Ok(Default::default())
    }
}

pub type Schema =
    juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}

use actix_web::web::Data;

pub async fn graphiql_route() -> Result<actix_web::HttpResponse, actix_web::Error> {
    juniper_actix::graphiql_handler("/graphql", None).await
}
pub async fn playground_route() -> Result<actix_web::HttpResponse, actix_web::Error> {
    juniper_actix::playground_handler("/graphql", None).await
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
