use accesso_core::contracts::repo::AccessTokenRepo;
use accesso_core::contracts::UnexpectedDatabaseError;
use accesso_core::models;
use sqlx::types::Uuid;

use crate::entities::AccessToken;
use crate::Database;

#[async_trait]
impl AccessTokenRepo for Database {
    async fn access_token_create(
        &self,
        token: models::AccessToken,
    ) -> Result<models::AccessToken, UnexpectedDatabaseError> {
        let token = AccessToken::from(token);
        Ok(sqlx::query_as!(
            AccessToken,
            // language=PostgreSQL
            r#"
            INSERT INTO access_tokens
                (token, scopes, expires_at, registration_id)
            VALUES ($1, $2, $3, $4)
            RETURNING access_tokens.*
            "#,
            token.token,
            &token.scopes,
            token.expires_at,
            token.registration_id
        )
        .fetch_one(&self.pool)
        .await?
        .into())
    }

    async fn access_tokens_list(
        &self,
    ) -> Result<Vec<models::AccessToken>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            AccessToken,
            // language=PostgreSQL
            r#"
            SELECT token, scopes, expires_at, registration_id
            FROM access_tokens
            "#
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
    }

    async fn access_tokens_list_for_registration(
        &self,
        registration_id: Uuid,
    ) -> Result<Vec<models::AccessToken>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            AccessToken,
            // language=PostgreSQL
            r#"
            SELECT token, scopes, expires_at, registration_id
            FROM access_tokens
            WHERE registration_id = $1
            "#,
            registration_id,
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
    }

    async fn access_tokens_delete_all_for_user(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<u64, UnexpectedDatabaseError> {
        Ok(sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE FROM access_tokens
            USING user_registrations
            WHERE user_registrations.id = access_tokens.registration_id
                AND user_registrations.user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }
}
