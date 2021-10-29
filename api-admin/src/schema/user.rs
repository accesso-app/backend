use async_graphql::validators::Email;
use async_graphql::*;

use super::user_registration::UserRegistration;
use accesso_app::Service;
use accesso_core::contracts::Repository;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct User {
    id: uuid::Uuid,
    email: String,
    canonical_email: String,
    first_name: String,
    last_name: String,
}

impl From<accesso_core::models::User> for User {
    fn from(user: accesso_core::models::User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            canonical_email: user.canonical_email,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }
}

#[ComplexObject]
impl User {
    async fn registrations(
        &self,
        context: &Context<'_>,
    ) -> async_graphql::Result<Vec<UserRegistration>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let apps = db.user_registration_list_for_user(self.id).await?;
        Ok(apps.into_iter().map(Into::into).collect())
    }
}

#[derive(Default)]
pub struct QueryUser;

#[Object]
impl QueryUser {
    async fn users(&self, context: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let list = db.user_list().await?;
        Ok(list.into_iter().map(Into::into).collect())
    }

    async fn user_by_email(
        &self,
        context: &Context<'_>,
        #[graphql(validator(Email))] email: String,
    ) -> async_graphql::Result<Option<User>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db.user_get_by_email(email).await?.map(Into::into))
    }

    async fn user_by_id(
        &self,
        context: &Context<'_>,
        user_id: uuid::Uuid,
    ) -> async_graphql::Result<Option<User>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db.user_get_by_id(user_id).await?.map(Into::into))
    }
}
