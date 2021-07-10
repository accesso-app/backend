use crate::generated::components::request_bodies;
use crate::generated::components::responses::{
    RegisterConfirmationFailed, RegisterConfirmationFailedError,
};
use crate::generated::paths::register_confirmation as confirm;
use accesso_core::app::registrator::RegisterConfirmError;
use actix_web::web;

pub async fn route(
    body: web::Json<request_bodies::RegisterConfirmation>,
    app: web::Data<accesso_app::App>,
) -> Result<confirm::Response, confirm::Error> {
    use accesso_core::app::registrator::{RegisterForm, Registrator};
    use confirm::Response;

    let form = RegisterForm {
        confirmation_code: body.confirmation_code.clone(),
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
        password: body.password.clone(),
    };

    app.registrator_confirm(form)
        .await
        .map_err(map_confirmation_error)?;

    Ok(Response::Created)
}

fn map_confirmation_error(error: RegisterConfirmError) -> confirm::Error {
    use RegisterConfirmError::{
        AlreadyActivated, CodeNotFound, EmailSenderError, InvalidForm, Unexpected,
    };
    use RegisterConfirmationFailed as Failure;

    match error {
        Unexpected(e) => e.into(),
        CodeNotFound => Failure {
            error: RegisterConfirmationFailedError::CodeInvalidOrExpired,
        }
        .into(),
        AlreadyActivated(e) => Failure {
            error: RegisterConfirmationFailedError::EmailAlreadyActivated(e.into()),
        }
        .into(),
        InvalidForm(e) => Failure {
            error: RegisterConfirmationFailedError::InvalidForm(e),
        }
        .into(),
        EmailSenderError(e) => eyre::Report::from(e).into(),
    }
}
