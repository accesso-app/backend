use accesso_core::contracts::{AccessTokenRepo, UnexpectedDatabaseError};
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
    (client_id, token, user_id, scopes, expires_at)
VALUES ($1, $2, $3, $4, $5)
RETURNING client_id, token, user_id, scopes, expires_at
            "#,
            token.client_id,
            token.token,
            token.user_id,
            &token.scopes,
            token.expires_at
        )
        .fetch_one(&self.pool)
        .await
        .map(Into::into)
        .map_err(sqlx_error_to_unexpected)
    }
}
