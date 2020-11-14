use actix_http::HttpMessage;
use actix_web::{
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web,
};
use futures::future::{err, ok};

#[derive(Debug)]
pub struct Session {
    pub user: accesso_core::models::User,
    pub token: String,
}

impl actix_web::FromRequest for Session {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        use accesso_core::app::session::{Session, SessionResolveError::Unexpected};

        let session_config = req.app_data::<web::Data<crate::cookie::SessionCookieConfig>>();
        let app = req.app_data::<web::Data<crate::App>>();

        if let (Some(session_config), Some(app)) = (session_config, app) {
            if let Some(ref cookie) = req.cookie(&session_config.name) {
                let app = app.read().unwrap();
                let token = cookie.value().to_owned();

                match app.session_resolve_by_cookie(token.clone()) {
                    Err(Unexpected) => err(ErrorInternalServerError(Null)),
                    Ok(None) => err(ErrorUnauthorized(Null)),
                    Ok(Some(user)) => ok(Self { user, token }),
                }
            } else {
                log::trace!("no cookie found");
                err(ErrorUnauthorized(Null))
            }
        } else {
            log::error!("failed to resolve crate::cookie::SessionCookieConfig or/and crate::App");
            err(ErrorInternalServerError(Null))
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct Null;

impl std::fmt::Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
