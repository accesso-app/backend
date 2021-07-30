use accesso_core::app::accesso_authorize::{AccessoAuthorize, UpdateAdminUserFailure};
use accesso_core::contracts::{SecureGenerator, Repository, AdminUserCreateError};
use accesso_core::models::{AdminUser, AdminSessionToken};
use crate::{App, Service};
use accesso_core::app::AdminUserInfo;

// const ACCESS_TOKEN_LENGTH: u8 = 60;

#[async_trait]
impl AccessoAuthorize for App {
    async fn authorize(&self, info: AdminUserInfo) -> Result<(AdminUser, AdminSessionToken), UpdateAdminUserFailure> {
        let db = self.get::<Service<dyn Repository>>()?;
        let generator = self.get::<Service<dyn SecureGenerator>>()?;

        let user = db.user_find_by_accesso(info.accesso_id).await?;

        let actual_user = if let Some(mut user) = user {
            user.first_name = info.first_name;
            user.last_name = info.last_name;
            Ok(db.user_update(user).await?)
        } else {
            match db.user_create(info.clone().into()).await {
                Ok(user) => Ok(user),
                Err(AdminUserCreateError::UnexpectedFailure(e)) => Err(UpdateAdminUserFailure::Unexpected(e)),

                // potentially impossible
                Err(err @ AdminUserCreateError::AdminUserAlreadyExists) => db
                    .user_find_by_accesso(info.accesso_id)
                    .await?
                    .ok_or_else(|| UpdateAdminUserFailure::Unexpected(err.into())),
            }
        }?;

        let token = generator.generate_token();
        let session_token = AdminSessionToken::new(actual_user.id, token);
        let token = db.admin_token_create(session_token).await?;

        Ok((actual_user, token))
    }
}
