use crate::generated::{
    components::{
        request_bodies::ApplicationGetRequestBody,
        responses::{
            ApplicationGetError as FailureVariant, ApplicationGetFailure as Failure,
            ApplicationGetSuccess,
        },
        schemas::Application,
    },
    paths::application_get::{Error, Response},
};
use accesso_core::app::application::{Application as _, ApplicationGetError};
use actix_web::web::{Data, Json};

pub async fn route(
    app: Data<accesso_app::App>,
    body: Json<ApplicationGetRequestBody>,
) -> Result<Response, Error> {
    let body = body.into_inner();
    let client = app
        .application_get(body.application_id)
        .await
        .map_err(map_client_get_error)?;

    Ok(Response::Ok(ApplicationGetSuccess {
        client: Application {
            id: client.id,
            title: client.title,
            allowed_registrations: client.allowed_registrations,
            // TODO: Add pictures for clients
            avatar: None,
        },
    }))
}

fn map_client_get_error(error: ApplicationGetError) -> Error {
    use ApplicationGetError::*;
    match error {
        Unexpected(e) => Error::InternalServerError(e),
        ApplicationNotFound => Error::BadRequest(Failure {
            error: FailureVariant::NotFound,
        }),
    }
}
