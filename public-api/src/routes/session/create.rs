use crate::generated::components::{request_bodies, responses};
use crate::generated::paths::session_create::Response;
use actix_swagger::Answer;
use actix_web::{cookie::CookieBuilder, web};

pub async fn route(
    body: web::Json<request_bodies::SessionCreate>,
    session_config: web::Data<crate::cookie::SessionCookieConfig>,
    app: web::Data<crate::App>,
) -> Answer<'static, Response> {
    use accesso_public_logic::app::session::{
        Session,
        SessionCreateError::{InvalidCredentials, InvalidForm, Unexpected},
        SessionCreateForm,
    };

    let form = SessionCreateForm {
        email: body.email.clone(),
        password: body.password.clone(),
    };

    let mut app = app.write().unwrap();

    match app.session_create(form) {
        Err(Unexpected) => Response::Unexpected.answer(),
        Err(InvalidForm) => Response::BadRequest(responses::SessionCreateFailed {
            error: responses::SessionCreateFailedError::InvalidForm,
        })
        .answer(),
        Err(InvalidCredentials) => Response::BadRequest(responses::SessionCreateFailed {
            error: responses::SessionCreateFailedError::InvalidCredentials,
        })
        .answer(),
        Ok((session_token, user)) => {
            log::trace!("generated session_token: {}", session_token.token);
            Response::Created(responses::SessionCreateSucceeded {
                first_name: user.first_name,
                last_name: user.last_name,
            })
            .answer()
            .cookie(
                CookieBuilder::new(session_config.name.clone(), session_token.token)
                    // .expires(session_token.expires_at.into())
                    .path(session_config.path.clone())
                    .secure(session_config.secure)
                    .http_only(session_config.http_only)
                    .finish(),
            )
        }
    }
}
