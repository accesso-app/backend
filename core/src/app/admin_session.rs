use crate::models::{AdminAccessToken, User};
use async_trait::async_trait;

pub use crate::contracts::repo::SessionCreateError as RepoError;
use crate::app::session::{SessionResolveError};
use uuid::Uuid;
use crate::contracts::SomeError;

#[async_trait]
pub trait AdminSession {
    async fn admin_session_resolve_by_cookie(
        &self,
        cookie: String,
    ) -> Result<Option<crate::models::AdminSession>, SessionResolveError>;

    async fn get_admin_access_token(
        &self,
        user_id: Uuid,
        // TODO change SessionResolveError
    ) -> Result<Option<AdminAccessToken>, SessionResolveError>;

    async fn user_get_by_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<User>, SomeError>;
}
