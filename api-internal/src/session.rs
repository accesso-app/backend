use actix_web::{
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web,
};
use futures::Future;
use std::pin::Pin;

#[derive(Debug)]
pub struct Session {
    pub user: accesso_core::models::User,
    pub token: String,
}

impl actix_web::FromRequest for Session {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        use accesso_core::app::session::{Session, SessionResolveError::Unexpected};

        let req = req.clone();

        Box::pin(async move {
            let req = req.clone();
            let session_config = req.app_data::<web::Data<accesso_app::SessionCookieConfig>>();
            let app = req.app_data::<web::Data<accesso_app::App>>();

            if let (Some(session_config), Some(app)) = (session_config, app) {
                if let Some(ref cookie) = req.clone().cookie(&session_config.name) {
                    let token = cookie.value().to_owned();

                    match app.session_resolve_by_cookie(token.clone()).await {
                        Err(Unexpected(_)) => Err(ErrorInternalServerError(Null)),
                        Ok(None) => Err(ErrorUnauthorized(Null)),
                        Ok(Some(user)) => Ok(Self { user, token }),
                    }
                } else {
                    tracing::warn!("No cookie found!");
                    Err(ErrorUnauthorized(Null))
                }
            } else {
                Err(ErrorInternalServerError(Null))
            }
        })
    }
}

#[derive(Debug, serde::Serialize)]
struct Null;

impl std::fmt::Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
