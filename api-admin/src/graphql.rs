use actix_web::HttpResponse;
use async_graphql::http::{graphiql_source, playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{Request, Response};

use accesso_app::Service;
use accesso_core::contracts::{Repository, SecureGenerator};

use crate::schema::AdminSchema;

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

pub async fn main(
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

pub async fn playground() -> actix_web::Result<actix_web::HttpResponse> {
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

pub async fn subscriptions(
    schema: actix_web::web::Data<AdminSchema>,
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
) -> actix_web::Result<HttpResponse> {
    async_graphql_actix_web::WSSubscription::start(Schema::clone(&*schema), &req, payload)
}
