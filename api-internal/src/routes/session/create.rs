use crate::generated::components::responses::SessionCreateFailedError;
use crate::generated::components::{request_bodies, responses};
use crate::generated::paths::session_create::{Error, Response};
use accesso_core::app::session::SessionCreateError;
use actix_web::{web, HttpRequest, Responder};
use eyre::WrapErr;

#[tracing::instrument(name = "/session/create", skip(app, req))]
pub async fn route(
    body: web::Json<request_bodies::SessionCreate>,
    session_config: web::Data<accesso_app::SessionCookieConfig>,
    app: web::Data<accesso_app::App>,
    req: HttpRequest,
) -> Result<impl Responder, Error> {
    use accesso_core::app::session::{Session, SessionCreateForm};

    let form = SessionCreateForm {
        email: body.email.clone(),
        password: body.password.clone(),
    };

    let (session_token, user) = app
        .session_create(form)
        .await
        .map_err(map_session_create_error)?;

    tracing::trace!(
        session_token = %session_token.token,
        "Generated session_token"
    );

    let mut response = Response::Created(responses::SessionCreateSucceeded {
        first_name: user.first_name,
        last_name: user.last_name,
    })
    .respond_to(&req);

    response
        .add_cookie(&session_config.to_cookie(session_token))
        .wrap_err("Could not add cookie")?;

    Ok(response)
}

fn map_session_create_error(error: SessionCreateError) -> Error {
    match error {
        SessionCreateError::Unexpected(e) => e.into(),
        SessionCreateError::InvalidForm(e) => {
            Error::BadRequest(responses::SessionCreateFailed { error: e.into() })
        }
        SessionCreateError::InvalidCredentials => {
            Error::BadRequest(responses::SessionCreateFailed {
                error: SessionCreateFailedError::InvalidCredentials,
            })
        }
    }
}
