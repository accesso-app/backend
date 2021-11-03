use accesso_core::contracts::repo::UserRepo;
use accesso_core::contracts::{
    RegisterUserError, UnexpectedDatabaseError, UserCredentials, UserEditError, UserEditForm,
    UserRegisterForm,
};
use accesso_core::models;

use crate::entities::User;
use crate::mappers::{sqlx_error_to_account_edit_error, sqlx_error_to_register_user_error};
use crate::Database;

#[async_trait]
impl UserRepo for Database {
    async fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError> {
        Ok(sqlx::query_scalar!(
            // language=PostgreSQL
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE canonical_email = $1) AS "exists!"
            "#,
            email.to_lowercase()
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn user_register(
        &self,
        form: UserRegisterForm,
    ) -> Result<models::User, RegisterUserError> {
        let user = User {
            id: uuid::Uuid::new_v4(),
            email: form.email.clone(),
            canonical_email: form.email.to_lowercase(),
            first_name: form.first_name,
            last_name: form.last_name,
            password_hash: form.password_hash.trim_end_matches('\u{0}').to_owned(),
        };

        sqlx::query!(
            // language=PostgreSQL
            r#"
            INSERT INTO users
                (id, email, canonical_email, first_name, last_name, password_hash)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user.id,
            user.email,
            user.canonical_email,
            user.first_name,
            user.last_name,
            user.password_hash
        )
        .execute(&self.pool)
        .await
        .map_err(sqlx_error_to_register_user_error)?;

        Ok(Into::into(user))
    }

    #[tracing::instrument(skip(self, creds), fields(creds.email = % creds.email))]
    async fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<models::User>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT id,
                   email,
                   password_hash,
                   first_name,
                   last_name,
                   canonical_email
            FROM users
            WHERE canonical_email = $1
            "#,
            creds.email.to_lowercase()
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn user_get_by_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Option<models::User>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT users.*
            FROM users
            WHERE users.id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn user_get_by_email(
        &self,
        email: String,
    ) -> Result<Option<models::User>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT users.*
            FROM users
            WHERE users.email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn user_edit_by_id(
        &self,
        user_id: uuid::Uuid,
        form: UserEditForm,
    ) -> Result<models::User, UserEditError> {
        let user = self
            .user_get_by_id(user_id)
            .await?
            .ok_or(UserEditError::UserNotFound)?;

        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            UPDATE users
            SET first_name = $2, last_name = $3, email = $4, canonical_email = $5
            WHERE id = $1
            RETURNING users.*
            "#,
            user_id,
            form.first_name.unwrap_or(user.first_name),
            form.last_name.unwrap_or(user.last_name),
            form.email.clone().unwrap_or(user.email),
            form.email
                .map(|email| email.to_lowercase())
                .unwrap_or(user.canonical_email),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_account_edit_error)
        .and_then(|option| option.ok_or(UserEditError::UserNotFound))
        .map(Into::into)?)
    }

    async fn user_list(&self) -> Result<Vec<models::User>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
                SELECT id,
                   email,
                   password_hash,
                   first_name,
                   last_name,
                   canonical_email
                FROM users 
                "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| UnexpectedDatabaseError::SqlxError(e))
        .map(|list| list.into_iter().map(Into::into).collect())?)
    }

    async fn user_search(
        &self,
        query: String,
    ) -> Result<Vec<models::User>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT id,
               email,
               password_hash,
               first_name,
               last_name,
               canonical_email
            FROM users
            WHERE email ILIKE $1
                OR first_name ILIKE $1
                OR last_name ILIKE $1
            "#,
            format!("%{}%", query),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| UnexpectedDatabaseError::SqlxError(e))?
        .into_iter()
        .map(Into::into)
        .collect())
    }
}
