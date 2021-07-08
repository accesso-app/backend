use accesso_core::contracts::{
    RegisterUserError, UnexpectedDatabaseError, UserCredentials, UserRegisterForm, UserRepo,
};
use accesso_core::models;

use crate::entities::User;
use crate::mappers::{sqlx_error_to_register_user_error, sqlx_error_to_unexpected};
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
        .await
        .map_err(sqlx_error_to_unexpected)?)
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

    #[tracing::instrument(skip(self, creds), fields(creds.email = %creds.email))]
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
        .await
        .map_err(sqlx_error_to_unexpected)?
        .map(Into::into))
    }
}
