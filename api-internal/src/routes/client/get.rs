use crate::generated::{
    components::{
        request_bodies::ClientGetRequestBody,
        responses::{
            ClientGetError as FailureVariant, ClientGetFailure as Failure, ClientGetSuccess,
        },
        schemas::Client,
    },
    paths::client_get::{Error, Response},
};
use accesso_core::app::client::{Client as _, ClientGetError};
use actix_web::web::{Data, Json};

pub async fn route(
    app: Data<accesso_app::App>,
    body: Json<ClientGetRequestBody>,
) -> Result<Response, Error> {
    let body = body.into_inner();
    let client = app
        .client_get(body.client_id)
        .await
        .map_err(map_client_get_error)?;

    Ok(Response::Ok(ClientGetSuccess {
        client: Client {
            id: client.id,
            title: client.title,
            allowed_registrations: client.allowed_registrations,
            // TODO: Add pictures for clients
            avatar: None,
        },
    }))
}

fn map_client_get_error(error: ClientGetError) -> Error {
    use ClientGetError::*;
    match error {
        Unexpected(e) => Error::InternalServerError(e),
        ClientNotFound => Error::BadRequest(Failure {
            error: FailureVariant::NotFound,
        }),
    }
}
