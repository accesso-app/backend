use actix_web::cookie::{Cookie, CookieBuilder};
use accesso_core::models::AdminSessionToken;

#[derive(Debug, Clone)]
pub struct AdminSessionCookieConfig {
    pub name: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
}

impl AdminSessionCookieConfig {
    pub fn to_cookie(&self, token: AdminSessionToken) -> Cookie<'static> {
        CookieBuilder::new(self.name.clone(), token.token)
            .expires(time::OffsetDateTime::from_unix_timestamp(
                token.expires_at.timestamp(),
            ))
            .path(self.path.clone())
            .secure(self.secure)
            .http_only(self.http_only)
            .finish()
    }
}
