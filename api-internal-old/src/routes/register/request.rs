use crate::generated::components::{request_bodies, responses};
use crate::generated::paths::register_request;
use accesso_core::app::registrator::RegisterRequestError;
use actix_web::web;

#[tracing::instrument(skip(app))]
pub async fn route(
    body: web::Json<request_bodies::Register>,
    app: web::Data<accesso_app::App>,
) -> Result<register_request::Response, register_request::Error> {
    use accesso_core::app::registrator::{CreateRegisterRequest, Registrator};
    use register_request::Response;

    let request = app
        .registrator_create_request(CreateRegisterRequest::from_email(&body.email))
        .await
        .map_err(map_register_request_error)?;

    Ok(Response::Created(responses::RegistrationRequestCreated {
        expires_at: request.expires_at.timestamp(),
    }))
}

#[allow(dead_code)]
fn map_register_request_error(error: RegisterRequestError) -> register_request::Error {
    use RegisterRequestError::{EmailAlreadyRegistered, EmailSenderError, InvalidForm, Unexpected};

    match error {
        Unexpected(e) => e.into(),
        EmailSenderError(e) => eyre::Report::from(e).into(),
        EmailAlreadyRegistered(email) => responses::RegisterFailed {
            error: responses::RegisterFailedError::EmailAlreadyRegistered(email),
        }
        .into(),
        InvalidForm(e) => responses::RegisterFailed {
            error: responses::RegisterFailedError::InvalidForm(e),
        }
        .into(),
    }
}
