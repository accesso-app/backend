use crate::entities::AdminSessionToken;
use crate::Database;
use sqlx::types::Uuid;
use accesso_core::contracts::{RepoResult, AdminSessionTokenRepo};
use accesso_core::models;

#[async_trait]
impl AdminSessionTokenRepo for Database {
    async fn admin_token_delete_by_user(&self, user_id: Uuid) -> RepoResult<u64> {
        Ok(sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE
            FROM admin_session_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }

    async fn admin_token_delete(&self, token: String) -> RepoResult<u64> {
        Ok(sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE 
            FROM admin_session_tokens
            WHERE token = $1
            "#,
            token
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }

    async fn admin_token_find(&self, token: String) -> RepoResult<Option<models::AdminSessionToken>> {
        Ok(sqlx::query_as!(
            AdminSessionToken,
            // language=PostgreSQL
            r#"
            SELECT user_id, token, expires_at
            FROM admin_session_tokens
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn admin_token_find_by_user(&self, user_id: Uuid) -> RepoResult<Option<models::AdminSessionToken>> {
        Ok(sqlx::query_as!(
            AdminSessionToken,
            // language=PostgreSQL
            r#"
            SELECT user_id, token, expires_at
            FROM admin_session_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn admin_token_create(&self, token: models::AdminSessionToken) -> RepoResult<models::AdminSessionToken> {
        Ok(sqlx::query_as!(
            AdminSessionToken,
            // language=PostgreSQL
            r#"
            INSERT INTO admin_session_tokens
                (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING user_id, token, expires_at
            "#,
            token.user_id,
            token.token,
            token.expires_at
        )
        .fetch_one(&self.pool)
        .await?
        .into())
    }
}
