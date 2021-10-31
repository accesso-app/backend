use crate::generated::{
    components::request_bodies,
    paths::session_delete::{Error, Response},
};
use crate::session::Session;
use accesso_core::app::session::SessionDeleteError;
use actix_web::http::header::SET_COOKIE;
use actix_web::http::HeaderValue;
use actix_web::{web, Responder};
use cookie::CookieBuilder;
use eyre::WrapErr;

pub async fn route(
    body: web::Json<request_bodies::SessionDelete>,
    session_config: web::Data<accesso_app::SessionCookieConfig>,
    app: web::Data<accesso_app::App>,
    session: Session,
) -> Result<impl Responder, Error> {
    use accesso_core::app::session::{Session, SessionDeleteStrategy};

    let strategy = match body.delete_all_sessions {
        true => SessionDeleteStrategy::All,
        false => SessionDeleteStrategy::Single(session.token.to_owned()),
    };

    app.session_delete(&session.user, strategy)
        .await
        .map_err(map_session_delete_error)?;

    let cookie = CookieBuilder::new(session_config.name.to_owned(), "")
        // TODO: extract to function or Trait
        .expires(time::OffsetDateTime::now_utc())
        .path(session_config.path.to_owned())
        .secure(session_config.secure)
        .http_only(session_config.http_only)
        .finish();

    let header_value = HeaderValue::from_str(&cookie.to_string())
        .wrap_err("Could not create header value for cookie!")?;

    Ok(Response::Ok.with_header((SET_COOKIE, header_value)))
}

fn map_session_delete_error(error: SessionDeleteError) -> Error {
    match error {
        SessionDeleteError::Unexpected(e) => e.into(),
    }
}
