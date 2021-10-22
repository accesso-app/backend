use actix_web::HttpResponse;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::{
    http::{graphiql_source, playground_source},
    Context, EmptyMutation, EmptySubscription, InputObject, Object, Schema, SchemaBuilder,
    SimpleObject,
};
use async_graphql_actix_web::{Request, Response};

use accesso_app::Service;
use accesso_core::contracts::{Repository, SecureGenerator};

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

    // async fn application(
    //     &self,
    //     context: &Context<'_>,
    //     id: uuid::Uuid,
    // ) -> async_graphql::Result<Option<Application>> {
    //     let db = context.data::<Service<dyn Repository>>()?;
    //     let found = db.application_find_by_id(id).await?;
    //
    //     Ok(found.map(|app| app.into()))
    // }
    //
    // async fn applications(&self, context: &Context<'_>) -> async_graphql::Result<Vec<Application>> {
    //     let db = context.data::<Service<dyn Repository>>()?;
    //     let list = db.application_list().await?;
    //     Ok(list.into_iter().map(|client| client.into()).collect())
    // }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn demo(&self) -> &'static str {
        "hello"
    }
}

type AdminSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription)
}

#[derive(Debug, serde::Serialize, thiserror::Error)]
pub enum Failure {
    #[error(transparent)]
    Failure(
        #[from]
        #[serde(skip)]
        eyre::Report,
    ),
}

impl actix_web::ResponseError for Failure {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError()
            .json(serde_json::json!({ "error": "internal_server_error" }))
    }
}

pub async fn graphql(
    schema: actix_web::web::Data<AdminSchema>,
    request: Request,
    app: actix_web::web::Data<accesso_app::App>,
) -> Result<Response, Failure> {
    let db = app.get::<Service<dyn Repository>>()?.clone();
    let generator = app.get::<Service<dyn SecureGenerator>>()?.clone();

    Ok(schema
        .execute(request.into_inner().data(db).data(generator))
        .await
        .into())
}

pub async fn graphql_playground() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/"),
        )))
}

pub async fn graphiql() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", Some("/"))))
}

pub async fn index_ws(
    schema: actix_web::web::Data<AdminSchema>,
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
) -> actix_web::Result<HttpResponse> {
    async_graphql_actix_web::WSSubscription::start(Schema::clone(&*schema), &req, payload)
}
