use async_graphql::validators::Email;
use async_graphql::*;

use super::user_registration::UserRegistration;
use accesso_app::Service;
use accesso_core::contracts::{Repository, SecureGenerator, UserEditForm};

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

    async fn access_tokens_count(&self, context: &Context<'_>) -> async_graphql::Result<u64> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db.user_access_tokens_count(self.id).await?)
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

    async fn users_search(
        &self,
        context: &Context<'_>,
        query: String,
    ) -> async_graphql::Result<Vec<User>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db
            .user_search(query)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

#[derive(InputObject)]
pub struct UserEdit {
    id: uuid::Uuid,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Default)]
pub struct MutationUser;

#[derive(SimpleObject, Clone)]
pub struct UserPasswordReset {
    password: String,
    user: User,
}

#[Object]
impl MutationUser {
    pub async fn user_edit(
        &self,
        context: &Context<'_>,
        user: UserEdit,
    ) -> async_graphql::Result<Option<User>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(Some(
            db.user_edit_by_id(
                user.id,
                UserEditForm {
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email,
                },
            )
            .await?
            .into(),
        ))
    }

    pub async fn user_password_reset(
        &self,
        context: &Context<'_>,
        user_id: uuid::Uuid,
    ) -> async_graphql::Result<Option<UserPasswordReset>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let generator = context.data::<Service<dyn SecureGenerator>>()?;
        let password = generator.generate_token();
        let password_hash = generator.password_hash(password.clone()).0;
        let user = db.user_password_reset(user_id, password_hash).await?;

        if let Some(user) = user {
            Ok(Some(UserPasswordReset {
                password,
                user: user.into(),
            }))
        } else {
            Ok(None)
        }
    }
}
