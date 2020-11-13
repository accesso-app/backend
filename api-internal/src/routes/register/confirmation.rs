use crate::generated::components::request_bodies;
use crate::generated::components::responses::{
    RegisterConfirmationFailed, RegisterConfirmationFailedError,
};
use crate::generated::paths::register_confirmation as confirm;
use actix_swagger::Answer;
use actix_web::web;

pub async fn route(
    body: web::Json<request_bodies::RegisterConfirmation>,
    app: web::Data<crate::App>,
) -> Answer<'static, confirm::Response> {
    use accesso_core::app::registrator::{
        RegisterConfirmError::{AlreadyActivated, CodeNotFound, InvalidForm, Unexpected},
        RegisterForm, Registrator,
    };
    use confirm::Response;

    let form = RegisterForm {
        confirmation_code: body.confirmation_code.clone(),
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
        password: body.password.clone(),
    };

    let mut app = app.write().unwrap();

    match app.registrator_confirm(form) {
        Err(Unexpected) => Response::Unexpected,
        Err(CodeNotFound) => Response::BadRequest(RegisterConfirmationFailed {
            error: RegisterConfirmationFailedError::CodeInvalidOrExpired,
        }),
        Err(AlreadyActivated) => Response::BadRequest(RegisterConfirmationFailed {
            error: RegisterConfirmationFailedError::EmailAlreadyActivated,
        }),
        Err(InvalidForm) => Response::BadRequest(RegisterConfirmationFailed {
            error: RegisterConfirmationFailedError::InvalidForm,
        }),
        Ok(()) => Response::Created,
    }
    .answer()
}
