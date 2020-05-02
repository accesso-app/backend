use crate::generated::components::{responses, schemas};
use crate::generated::paths::session_get::Response;
use actix_http::HttpMessage;
use actix_swagger::Answer;
use actix_web::web;

pub async fn route(
    session_config: web::Data<crate::cookie::SessionCookieConfig>,
    app: web::Data<crate::App>,
    req: web::HttpRequest,
) -> Answer<'static, Response> {
    use authmenow_public_logic::app::session::{Session, SessionResolveError::Unexpected};

    let app = app.read().unwrap();

    if let Some(ref cookie) = req.cookie(&session_config.name) {
        match app.session_resolve(cookie.value().to_owned()) {
            Err(Unexpected) => Response::Unexpected.answer(),
            Ok(None) => Response::Unauthorized.answer(),
            Ok(Some(user)) => Response::Ok(responses::SessionGetSuccess {
                user: schemas::SessionUser {
                    first_name: user.first_name,
                    last_name: user.last_name,
                },
            })
            .answer(),
        }
    } else {
        Response::Unauthorized.answer()
    }
}
