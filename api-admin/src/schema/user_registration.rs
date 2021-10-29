use async_graphql::{ComplexObject, Context, SimpleObject};

use super::{access_token::AccessToken, application::Application, user::User};
use accesso_app::Service;
use accesso_core::contracts::Repository;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct UserRegistration {
    pub id: uuid::Uuid,
    /// Field renamed from `client_id`
    pub application_id: uuid::Uuid,
    // User registration does not expires!
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user_id: uuid::Uuid,
}

impl From<accesso_core::models::UserRegistration> for UserRegistration {
    fn from(ur: accesso_core::models::UserRegistration) -> Self {
        Self {
            id: ur.id,
            application_id: ur.client_id,
            created_at: ur.created_at,
            user_id: ur.user_id,
        }
    }
}

#[ComplexObject]
impl UserRegistration {
    async fn user(&self, context: &Context<'_>) -> async_graphql::Result<Option<User>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let user = db.user_get_by_id(self.user_id).await?;
        Ok(user.map(Into::into))
    }

    async fn application(
        &self,
        context: &Context<'_>,
    ) -> async_graphql::Result<Option<Application>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let application = db.application_find_by_id(self.application_id).await?;
        Ok(application.map(Into::into))
    }

    async fn access_tokens(
        &self,
        context: &Context<'_>,
    ) -> async_graphql::Result<Vec<AccessToken>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db
            .access_tokens_list_for_registration(self.id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
