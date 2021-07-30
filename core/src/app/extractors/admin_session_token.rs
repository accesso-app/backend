use crate::app::error::AdminSessionTokenExtractorError;
use actix_web::dev::Payload;
use actix_web::error::Error;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

/// Extractor for the session token. Right now works only for cookies.
///
/// # Examples
/// ```
/// use cardbox_core::app::extractors::AdminSessionToken;
/// use actix_web::get;
///
/// #[get("/")]
/// async fn index(session_token: AdminSessionToken) -> String {
///     format!("Request from user with session token: {}!", session_token.into_inner())
/// }
/// ```
#[derive(Debug)]
pub struct AdminSessionToken(String);

impl FromRequest for AdminSessionToken {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        req
            // TODO: extract session token name into config
            .cookie("admin-session-token")
            .map(|cookie| ok(AdminSessionToken(cookie.value().to_string())))
            .unwrap_or_else(move || {
                let e = AdminSessionTokenExtractorError::NoSessionToken;

                tracing::debug!(
                    cookies = ?req.cookies(),
                    "Failed during AdminSessionToken extractor from cookies"
                );

                err(e.into())
            })
    }
}

impl AdminSessionToken {
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod test {
    use crate::app::error::AdminSessionTokenExtractorError;
    use crate::app::extractors::AdminSessionToken;
    use actix_web::cookie::Cookie;
    use actix_web::dev::Payload;
    use actix_web::FromRequest;

    #[actix_rt::test]
    async fn session_token_extracts_correctly() -> Result<(), actix_web::Error> {
        let req = actix_web::test::TestRequest::get()
            .cookie(Cookie::new("session-token", "mytoken"))
            .to_http_request();

        let session_token = AdminSessionToken::from_request(&req, &mut Payload::None).await?;
        assert_eq!(session_token.into_inner(), "mytoken".to_string());
        Ok(())
    }

    #[actix_rt::test]
    async fn returns_error_if_no_session_token_cookie() -> Result<(), actix_web::Error> {
        let req = actix_web::test::TestRequest::get()
            .cookie(Cookie::new("not-a-session-token", "mytoken"))
            .to_http_request();

        let session_token = AdminSessionToken::from_request(&req, &mut Payload::None).await;

        session_token
            .err()
            .map(|e| {
                assert!(matches!(
                    e.as_error::<AdminSessionTokenExtractorError>(),
                    Some(&AdminSessionTokenExtractorError::NoAdminSessionToken)
                ))
            })
            .unwrap();

        Ok(())
    }
}
