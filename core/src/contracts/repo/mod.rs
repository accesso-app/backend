pub use access_token::*;
pub use application::*;
pub use auth_code::*;
pub use requests::*;
pub use session::*;
pub use user::*;
pub use user_registration::*;

mod access_token;
mod application;
mod auth_code;
mod requests;
mod session;
mod user;
mod user_registration;

#[derive(Debug, thiserror::Error)]
pub enum UnexpectedDatabaseError {
    #[error("Unexpected database error: {0}")]
    SqlxError(#[from] sqlx_core::error::Error),
}

#[cfg(feature = "testing")]
pub struct MockDb {
    pub users: MockUserRepo,
    pub requests: MockRequestsRepo,
    pub session: MockSessionRepo,
    pub auth_code: MockAuthCodeRepo,
    pub application: MockApplicationRepo,
    pub access_token: MockAccessTokenRepo,
    pub user_registrations: MockUserRegistrationsRepo,
}

#[cfg(feature = "testing")]
impl Default for MockDb {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "testing")]
impl MockDb {
    pub fn new() -> Self {
        Self {
            users: MockUserRepo::new(),
            requests: MockRequestsRepo::new(),
            session: MockSessionRepo::new(),
            access_token: MockAccessTokenRepo::new(),
            auth_code: MockAuthCodeRepo::new(),
            application: MockApplicationRepo::new(),
            user_registrations: MockUserRegistrationsRepo::new(),
        }
    }
}
