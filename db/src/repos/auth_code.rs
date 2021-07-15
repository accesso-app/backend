use accesso_core::contracts::repo::AuthCodeRepo;
use accesso_core::contracts::UnexpectedDatabaseError;
use accesso_core::models;

use crate::entities::AuthorizationCode;
use crate::Database;

#[async_trait]
impl AuthCodeRepo for Database {
    async fn auth_code_create(
        &self,
        code: models::AuthorizationCode,
    ) -> Result<models::AuthorizationCode, UnexpectedDatabaseError> {
        let code = AuthorizationCode::from(code);

        Ok(sqlx::query_as!(
            AuthorizationCode,
            // language=PostgreSQL
            r#"
            INSERT INTO authorization_codes (client_id, code, created_at, redirect_uri, scope, user_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING client_id, code, created_at, redirect_uri, scope, user_id
            "#,
            code.client_id,
            code.code,
            code.created_at,
            code.redirect_uri,
            code.scope.as_deref(),
            code.user_id
        )
        .fetch_one(&self.pool)
        .await
        .map(Into::into)?)
    }

    async fn auth_code_read(
        &self,
        code: String,
    ) -> Result<Option<models::AuthorizationCode>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            AuthorizationCode,
            // language=PostgreSQL
            r#"
            SELECT client_id, code, created_at, redirect_uri, scope, user_id
            FROM authorization_codes
            WHERE code = $1
            "#,
            code
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }
}
