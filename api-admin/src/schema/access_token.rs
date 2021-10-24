use super::user_registration::UserRegistration;
use accesso_app::Service;
use accesso_core::contracts::Repository;
use async_graphql::{ComplexObject, Context, Object, SimpleObject};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AccessToken {
    token: String,
    scopes: Vec<String>,
    expires_at: chrono::DateTime<chrono::Utc>,
    registration_id: uuid::Uuid,
}

impl From<accesso_core::models::AccessToken> for AccessToken {
    fn from(token: accesso_core::models::AccessToken) -> Self {
        Self {
            token: token.token,
            scopes: token.scopes,
            expires_at: token.expires_at,
            registration_id: token.registration_id,
        }
    }
}

#[ComplexObject]
impl AccessToken {
    async fn registration(
        &self,
        context: &Context<'_>,
    ) -> async_graphql::Result<Option<UserRegistration>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db
            .user_registration_get_by_id(self.registration_id)
            .await?
            .map(Into::into))
    }
}

#[derive(Default)]
pub struct QueryAccessToken;

#[Object]
impl QueryAccessToken {
    async fn access_tokens(
        &self,
        context: &Context<'_>,
    ) -> async_graphql::Result<Vec<AccessToken>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db
            .access_tokens_list()
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
