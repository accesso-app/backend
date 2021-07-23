use async_trait::async_trait;
use accesso_core::app::admin_session::AdminSession;
use crate::{App, Service};
use accesso_core::models::{User, AdminAccessToken};
use accesso_core::app::session::SessionResolveError;
use accesso_core::contracts::{Repository, GetUserBySessionError, SomeError};
use uuid::Uuid;

#[async_trait]
impl AdminSession for App {
    async fn admin_session_resolve_by_cookie(&self, cookie: String) -> Result<Option<accesso_core::models::AdminSession>, SessionResolveError> {
        let db = self.get::<Service<dyn Repository>>()?;

        match db.get_admin_session_token(cookie).await {
            Err(GetUserBySessionError::Unexpected(e)) => Err(SessionResolveError::Unexpected(e)),
            Err(GetUserBySessionError::NotFound) => Ok(None),
            Ok(user) => Ok(Some(user)),
        }

    }

    async fn get_admin_access_token(&self, user_id: Uuid) -> Result<Option<AdminAccessToken>, SessionResolveError> {
        let db = self.get::<Service<dyn Repository>>()?;

        match db.get_access_token(user_id).await {
            Err(GetUserBySessionError::Unexpected(e)) => Err(SessionResolveError::Unexpected(e)),
            Err(GetUserBySessionError::NotFound) => Ok(None),
            Ok(token) => Ok(Some(token)),
        }
    }

    //TODO Change SessionResolveError to actual one
    async fn user_get_by_id(&self, id: Uuid) -> Result<Option<User>, SomeError> {
        let db = self.get::<Service<dyn Repository>>()?;

        match db.user_get_by_id(id).await {
            Err(_) => Err(SomeError::DEFAULT),
            Ok(user) => Ok(user),
        }
    }


}
