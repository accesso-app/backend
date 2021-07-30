use async_trait::async_trait;
use crate::{App, Service};
use accesso_core::contracts::{Repository};
use accesso_core::app::admin_session::{AdminSession, AdminSessionResolveError};

#[async_trait]
impl AdminSession for App {
    async fn admin_session_token_get(&self, cookie: String) -> Result<Option<accesso_core::models::AdminUser>, AdminSessionResolveError> {
        let db = self.get::<Service<dyn Repository>>()?;
        let token = db.admin_token_find(cookie).await?;
        let user_id = token.and_then(|token| (!token.is_expired())
            .then(|| token))
            .map(|token| token.user_id);
        if let Some(user_id) = user_id {
            let user = db.user_find_by_id(user_id).await?;
            Ok(user)
        } else {
            Err(AdminSessionResolveError::Unauthorized)
        }
    }

    //TODO Change SessionResolveError to actual one
    // async fn user_get_by_id(&self, id: Uuid) -> Result<Option<User>, SomeError> {
    //     let db = self.get::<Service<dyn Repository>>()?;
    //
    //     match db.user_get_by_id(id).await {
    //         Err(_) => Err(SomeError::DEFAULT),
    //         Ok(user) => Ok(user),
    //     }
    // }

}
