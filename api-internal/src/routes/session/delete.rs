use crate::cookie::SessionCookieConfig;
use crate::generated::{
    components::request_bodies,
    paths::session_delete::{Answer, Response},
};
use crate::session::Session;
use actix_web::{cookie::CookieBuilder, web};

pub async fn route(
    body: web::Json<request_bodies::SessionDelete>,
    session_config: web::Data<SessionCookieConfig>,
    app: web::Data<crate::App>,
    session: Session,
) -> Answer {
    use accesso_core::app::session::{Session, SessionDeleteError, SessionDeleteStrategy};
    let mut app = app.write().unwrap();

    let strategy = match body.delete_all_sessions {
        true => SessionDeleteStrategy::All,
        false => SessionDeleteStrategy::Single(session.token.to_owned()),
    };

    match app.session_delete(&session.user, strategy) {
        Err(SessionDeleteError::Unexpected) => Response::Unexpected.into(),
        Ok(()) => Response::Ok.answer().cookie(
            CookieBuilder::new(session_config.name.to_owned(), "")
                // TODO: extract to function or Trait
                .expires(time::at(time::Timespec::new(
                    chrono::Utc::now().timestamp(),
                    0,
                )))
                .path(session_config.path.to_owned())
                .secure(session_config.secure)
                .http_only(session_config.http_only)
                .finish(),
        ),
    }
}
