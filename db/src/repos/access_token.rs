use accesso_core::contracts::repo::AccessTokenRepo;
use accesso_core::contracts::UnexpectedDatabaseError;
use accesso_core::models;

use crate::entities::AccessToken;
use crate::mappers::sqlx_error_to_unexpected;
use crate::Database;

#[async_trait]
impl AccessTokenRepo for Database {
    async fn access_token_create(
        &self,
        token: models::AccessToken,
    ) -> Result<models::AccessToken, UnexpectedDatabaseError> {
        let token = AccessToken::from(token);
        sqlx::query_as!(
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
        .await
        .map(Into::into)
        .map_err(sqlx_error_to_unexpected)
    }
}
