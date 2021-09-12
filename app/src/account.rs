use async_trait::async_trait;
use uuid::Uuid;

use accesso_core::{app::account, contracts::Repository, contracts::UserEditForm, models::User};

use crate::{App, Service};

#[async_trait]
impl account::Account for App {
    async fn account_edit(
        &self,
        user_id: Uuid,
        form: account::AccountEditForm,
    ) -> Result<User, account::AccountEditError> {
        let db = self.get::<Service<dyn Repository>>()?;

        let updated_user = db
            .user_edit_by_id(
                user_id,
                UserEditForm {
                    first_name: form.first_name,
                    last_name: form.last_name,
                },
            )
            .await?;

        Ok(updated_user)
    }
}
